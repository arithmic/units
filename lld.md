# UNITS Execution Environment 

## 1. Overview

This document presents a revised architecture for the UNITS execution environment. The design moves towards a more generic, scalable, and robust system by introducing a centralized Scheduler responsible for transaction batching, execution, and proof generation.

The core principles of the original design—stateless contracts, a key-value storage model, and ZK-provability—are retained. However, this new model abstracts the token logic further, enabling arbitrary code execution compiled to RISC-V for the SP1 ZK-VM.

**Key Design Goals:**

- **Generic Execution:** Support any token function e.g. mint, burn, and transfer through a single execute entry point.
- **Batch Processing:** Aggregate multiple transactions into a single block for efficient state updates and proof generation.
- **Block-Level ZK Proofs:** Generate one ZK proof for the entire block's execution, proving the validity of all state transitions within it.
- **Transactional Integrity:** Provide Merkle proofs for each transaction's inclusion within a block, ensuring verifiability.
- **Decoupled Architecture:** Separate the concerns of transaction ordering (Scheduler) and transaction execution (ZK-VM Client).

---

## 2. System Architecture

The system is composed of three main components: the **User**, the **Scheduler**, and the **Execution Client**.

- **User:** Submits a transaction specifying the target token, the function to execute, and the input data.

- **Scheduler:** A trusted server that:
  - Accepts and validates incoming transactions.
  - Maintains a transaction pool.
  - Selects a fixed number of transactions to form a Block.
  - Passes the block to the Execution Client.
  - **Persists the global state in a RocksDB instance.**
  - Stores the resulting state changes, ZK proofs, and transaction membership proofs.

- **Execution Client (SP1 ZK-VM):**
  - Receives a block of transactions from the Scheduler.
  - Executes the `execute` function for each transaction in the block sequentially within the ZK-VM.
  - Generates a single ZK proof attesting to the correctness of the entire block's execution.
  - Returns the final state writes and the ZK proof to the Scheduler.

---

## 3. Transaction Lifecycle & Status

A transaction progresses through a clearly defined lifecycle, which can be queried via the API.

- **pending:** The transaction has been submitted by the user and is waiting in the Scheduler's transaction pool.
- **executed:** The Scheduler has included the transaction in a block and sent it for execution. The transaction is now associated with a `block_id` and a Merkle proof of its inclusion in that block is available.
- **proof_generated:** The Execution Client has successfully generated the ZK proof for the entire block containing the transaction.
- **verified_onchain:** The block's ZK proof has been successfully verified on a settlement layer like Ethereum or Solana.

---

## 4. Core Data Structures

### Transaction

This is the fundamental object submitted by a user.

```rust
pub struct Transaction {
    // Unique identifier for the transaction (hash of the fields below)
    pub transaction_hash: [u8; 32],
    // The name or ID of the token contract to call
    pub token_id: String,
    // The name of the function to execute (e.g., "mint", "transfer", "custom_logic")
    pub function: String,
    // Serialized input data for the function
    pub input: Vec<u8>,
    // A sequential number for the signer's account
    pub nonce: u64,
    // The user's signature over the transaction data (token_id, function, input, nonce)
    pub signature: Vec<u8>,
}
```

### Block

A collection of transactions bundled by the Scheduler for execution.

```rust
pub struct Block {
    pub block_id: u64,
    pub transactions: Vec<Transaction>,
    // The Merkle root of the transactions in this block
    pub transaction_merkle_root: [u8; 32],
    // The state root before this block was executed
    pub pre_state_root: [u8; 32],
    // The state root after this block was executed
    pub post_state_root: [u8; 32],
    // The single ZK proof for the execution of all transactions in this block
    pub execution_proof: Vec<u8>,
}
```

### Transaction Receipt

The output of a single transaction's execution, containing only the state changes.

```rust
// Key-Value Storage Foundation
pub type StateKey = [u8; 32];
pub type StateValue = [u8; 32];

#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: StateKey,
    pub value: StateValue,
}

#[derive(Clone, Debug)]
pub struct TransactionReceipt {
    // The state changes (writes) produced by this transaction
    pub writes: Vec<KeyValue>,
}
```

---

## 5. Token Contract Interface

The `TokenContract` trait is simplified to a single, powerful `execute` function. Each token contract implements this trait, and the entire implementation is compiled to RISC-V.

### ExecutionContext

The `ExecutionContext` provides the environment for contract execution. 

```rust
#![no_std]
use core::result::Result;

pub type Address = [u8; 32];

#[derive(Clone, Debug)]
pub enum TokenError {
    InvalidInput,
    FunctionNotFound,
    Unauthorized,
    StateMismatch,
    InvalidNonce,
    Custom(u8),
}

pub type TokenResult<T> = Result<T, TokenError>;

// The context provided to the execute function
#[derive(Clone, Debug)]
pub struct ExecutionContext<'a> {
    pub signer: Address,
    pub pre_state: &'a [KeyValue], // Relevant key-value pairs loaded by the runtime
    pub timestamp: u64,
    pub block_id: u64,
    pub transaction_hash: [u8; 32],
    pub token_id: String,
    pub nonce: u64,
}
```

### TokenContract Trait

```rust
pub trait TokenContract {
    /// The main entry point for all token interactions.
    /// This function routes calls to the appropriate internal logic based on the `function` name.
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt>;
}
```

### Example Token Implementation

This conceptual example shows how a token would implement the `execute` function to handle different operations, including nonce checking.

```rust
pub struct MyToken;

impl TokenContract for MyToken {
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        // Nonce check (provable, must be first)
        let current_nonce = get_nonce_from_pre_state(ctx.signer, ctx.pre_state);
        if ctx.nonce != current_nonce {
            return Err(TokenError::InvalidNonce);
        }
        // Prepare the nonce increment write
        let nonce_key = hash_nonce_key(ctx.signer);
        let nonce_write = KeyValue { key: nonce_key, value: (current_nonce + 1).to_le_bytes() };

        let mut receipt = match function {
            "mint" => self.mint(ctx, input),
            "burn" => self.burn(ctx, input),
            "transfer" => self.transfer(ctx, input),
            "approve" => self.approve(ctx, input),
            _ => Err(TokenError::FunctionNotFound),
        }?;

        // Always include the nonce increment in the writes
        let mut writes = receipt.writes;
        writes.push(nonce_write);
        Ok(TransactionReceipt { writes })
    }
}

impl MyToken {
    // Internal function to handle minting logic
    fn mint(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        // 1. Deserialize input bytes to get minting parameters (e.g., recipient, amount).
        // 2. Perform authorization checks (e.g., is ctx.signer the issuer?).
        // 3. Check pre_state to ensure recipient doesn't already have a token.
        // 4. Construct the KeyValue writes for the new token.
        // 5. Return the TransactionReceipt with the writes.
        unimplemented!()
    }

    // Internal function for burning
    fn burn(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        // ... implementation ...
        unimplemented!()
    }

    // Internal function for transferring
    fn transfer(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        // ... implementation ...
        unimplemented!()
    }

    // Internal function for a custom action
    fn approve(&self, ctx: &ExecutionContext, input: &[u8]) -> TokenResult<TransactionReceipt> {
        // ... implementation ...
        unimplemented!()
    }
}

// Helper functions (conceptual)
fn get_nonce_from_pre_state(signer: Address, pre_state: &[KeyValue]) -> u64 {
    // Find the nonce key for the signer in pre_state and return its value as u64
    unimplemented!()
}
fn hash_nonce_key(signer: Address) -> [u8; 32] {
    // Hash function to derive the nonce key for the signer
    unimplemented!()
}
```

---

## 6. State and Proof Management

### Global State

The global state is a Sparse Merkle Tree (SMT) that maps user addresses to their respective data. This structure is ZK-friendly and allows for efficient state commitments.

- **Global SMT:** The root of this tree (`pre_state_root` and `post_state_root`) represents the entire state of the system. Each leaf in this tree corresponds to a user account.
  - `Key`: `hash(user_address)`
  - `Value`: `user_data_merkle_root`

- **User Data Merkle Tree:** Each user's data is itself a Merkle tree. This allows a user to prove ownership of a specific token without revealing their entire portfolio.
  - The leaves of this tree represent individual tokens or other user-specific state, like their nonce.
  - For example, a leaf could be `hash(token_id, token_metadata)`.
  - This nested structure enables privacy and selective disclosure. When a transaction is executed, only the relevant branches (or "witnesses") from the global SMT and the user's data tree need to be provided to the ZK-VM.

### Proof Generation

Two distinct proofs are generated during block processing:

- **ZK Execution Proof:** A single, comprehensive ZK proof generated by the SP1 client. It proves that all transactions within a block were executed correctly according to their respective contract logic, transitioning the global state from `pre_state_root` to `post_state_root`.
- **Transaction Membership Proof:** A standard Merkle proof for each transaction. It proves that a specific transaction was included in the block's `transaction_merkle_root`. This proof is lightweight and does not involve ZK cryptography.

---

## 7. Nonce and Signature Handling

### Nonce Handling

**Replay protection** is enforced by requiring each transaction to include a `nonce` and by storing the latest nonce for each signer in the state.

- **Scheduler (Pre-Execution Check):**
  - The Scheduler maintains the last committed nonce for each account.
  - When a new transaction arrives, it checks that `transaction.nonce == last_committed_nonce + 1`.
  - This provides fast feedback and spam prevention, but is not provable.

- **ZK-VM Execution (Provable Enforcement):**
  - The contract logic must check that the transaction's nonce matches the current nonce in state.
  - If the check fails, the transaction is rejected.
  - The contract must increment the nonce in the state as part of the transaction's writes.
  - This is enforced and proven as part of the ZK proof.

### Signature Handling

- The `signer` field in `ExecutionContext` is derived from the transaction's signature.
- The signature must cover all relevant transaction fields (`token_id`, `function`, `input`, `nonce`).
- The Scheduler verifies the signature before accepting the transaction.
- The ZK-VM may also verify the signature as part of execution, if required for full provability.

---

## 8. API Endpoints

### Get Transaction Status

Provides detailed information about a single transaction, including its status and proof of membership if it has been included in a block.

**Request:** `GET /transaction/{transaction_hash}`

**Response:**
```json
{
  "transaction_hash": "0x123...",
  "status": "executed",
  "details": {
    "token_id": "MyCoolToken",
    "function": "transfer",
    "input_args": {
      "to": "0xabc...",
      "amount": 100
    }
  },
  "block_id": 42,
  "membership_proof": {
    "root": "0xdef...",
    "siblings": ["0x..."]
  }
}
```

### Get Block Status

Provides the ZK proof for an entire block's execution. This proof can be submitted to a settlement layer for verification.

**Request:** `GET /block/{block_id}`

**Response:**
```json
{
  "block_id": 42,
  "status": "proof_generated",
  "transaction_merkle_root": "0xdef...",
  "pre_state_root": "0xaaa...",
  "post_state_root": "0xbbb...",
  "zk_execution_proof": "0x..."
}
```

