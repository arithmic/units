# UNITS Core Design Spec 

Objective: An Application talks to UNITS Core → Workflow, which performs Web2‑grade ACL/Policy checks, then schedules stateless Kernel Modules that enforce cryptographic signature verification over canonical messages. The system favors parallel, asynchronous execution with pessimistic locks over the object store.

# 1\. Scope & Non‑Goals

**In‑scope**

* Application ↔ Workflow ↔ Kernel interfaces (Web2 ACLs, cryptogrpahic signatures)  
* Prefetch‑reads execution model, mutation plan output  
* Async scheduling, WAL, CAS with short‑lived write locks  
* Receipts/auditing binding Web2 principals ↔ Web3 keys

**Non‑Goals**

* Consensus/networking, Validity Proof systems, global state commitments

# 2\. Components and Roles

## Application Layer

A real‑world entity (tenant/user/service) identifies itself using existing Web2 identity schemes like **JWT/OIDC** and, alongside that, supplies **cryptographic signature bundle(s)** that UNITS Core can verify. The application sends instructions naming a controller/function plus typed input, identity (JWT), and signature bundles.

## UNITS Core

### Workflow Layer

* **Identity & Policy:** Validates the JWT/OIDC identity, evaluates **ACL/ABAC** policies, and maintains a **directory that maps real‑world principals → allowed public keys** for downstream signature checks.

* **Plan:** Builds **readset/writeset** for each Instruction, where the Instruction is a function call inside the Kernel module. This is where **prefetching reads** occur to prepare inputs for kernel execution.

* **Asynch Executor:** Submits steps to an async executor that runs the kernel. After the kernel returns a MutationPlan, the workflow **applies updates under write locks** and records a receipt; locks are then released.

* **Receipts/Audit:** Persists a `TransactionReceipt` containing policy snapshot IDs and verified signatures, and Merkle Path corresponding to the Merkle tree of the bloc in which the transaction was included.

* **CAS \+ Locks when Applying Mutations (your model):** When the kernel outputs a MutationPlan (diff), the workflow applies each change with **compare‑and‑swap against per‑key versions** while holding the appropriate locks—preventing lost updates and ensuring atomicity within the step. (This extends the spec’s pessimistic‑locking semantics with the CAS layer.)

### Kernel Modules

* **Stateless verification & domain logic only:** The kernel **verifies cryptographic signatures** over the canonical message and **validates domain rules**. It is important to note that it **does not directly write state** but returns a **MutationPlan** describing intended updates. 

### StateStore

A key‑value store with **R/W locks**, **WAL‑backed durability**, recovery, and **per‑key monotonic u64 versions** to support CAS when applying MutationPlans. Locking is done in **canonical order**; writes become visible when locks are released.

# 3\. Data Model & Namespacing 

* StateKey: 32‑byte key.  
* `Version`: `u64` per key (incremented on every successful write).

---

# 4\. Instruction

struct Instruction {

token\_id: String,

function: String,

input: Bytes,

signatures: Vec\<SignatureBundle\>,

nonce: u64,

Idempotency\_key: u64

ttl\_ms: u32

}

---

# 5\. Execution Context

struct ExecutionContext\<'a\> {

input: &'a \[u8\],

reads: &'a \[ReadEntry\], // prefetched by Workflow

logger: &'a dyn Logger,

signatures: &'a \[SignatureBundle\],

}

struct ReadEntry {

key: StateKey,

value: Option\<Vec\<u8\>\>, // None if absent

version: u64, // Version at read time

}

---

# 6\. Kernel Modules

fn execute(function, ctx: \&ExecutionContext) \-\> Result\<MutationPlan, KernelError\>;

struct MutationPlan {

expected: Vec\<ExpectedVersion\>, // CAS guards derived from reads

ops: Vec\<MutationOp\>, // write intents

}

struct ExpectedVersion { key: StateKey, version: u64 }

enum MutationOp { Put { key: StateKey, value: Vec\<u8\> }, Del { key: StateKey } }

---

# 7\. State and Storage Interfaces

pub trait StateStore {  
fn get(\&self, key: \&StateKey) \-\> (Option\<Vec\<u8\>\>, u64); // returns (value, version)  
fn write\_batch(\&self, ops: &\[(MutationOp)\], cas: &\[(ExpectedVersion)\]) \-\> Result\<u64, StorageError\>;  
// returns a batch sequence number or LSN  
}

pub trait LockManager { /\* acquire/release W locks; canonical ordering; leases \*/ }  
pub trait WriteAheadLog { /\* append, iterate, rollback \*/ }

---

# 8\. Concurrency and Parallelism

* Kernel execution holds **no DB locks** → longer CPU work does not block writers.  
* Short‑lived W locks during apply; canonical ordering prevents deadlocks.  
* Scheduler favors steps with disjoint write sets; uses try‑locks \+ backoff.  
* Sharding by key prefix/object id remains applicable.

---

# 7\. End‑to‑End Flow (Example: Token transfer)

1. **Application** submits `Instruction` with JWT \+ signature.  
2. **Workflow** authenticates and authorizes; plans readset `{bal(sender), bal(receiver), token_meta}`.  
3. **Prefetch** those keys with versions → build `ExecutionContext`.  
4. **Kernel** verifies signature(s), computes diff: `{Put(bal_s, new), Put(bal_r, new)}`, `expected` carries versions for both balances and meta.  
5. **Workflow** acquires `W` locks for those keys, runs CAS write\_batch; on success increments versions and persists WAL;   
   1. On CAS mismatch, it re‑reads and may **re‑execute kernel** with the new `reads` (or abort with conflict depending on policy).  
6. Groups the executed transactions into blocks and emits **TransactionReceipt for** each transaction in the block.  
7. Sends the entire block with required data to be proved by the Proving Engine

