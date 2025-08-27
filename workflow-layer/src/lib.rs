//! Workflow Layer - UNITS Core Architecture
//!
//! This crate implements the Workflow Layer component of the UNITS Core design.
//! The Workflow Layer orchestrates transaction processing between the Application Layer and Kernel Modules.
//!
//! ## UNITS Core Architecture
//!
//! The Workflow Layer sits between the Application Layer and Kernel Modules:
//! - **Application Layer** → submits Instructions with JWT + signatures  
//! - **Workflow Layer** → handles identity/policy, planning, async execution, and receipts
//! - **Kernel Modules** → perform stateless verification and return MutationPlans
//!
//! ## Workflow Layer Responsibilities
//!
//! - **Identity & Policy**: JWT/OIDC validation, ACL/ABAC evaluation
//! - **Planning**: Build readset/writeset, prefetch data for kernel execution  
//! - **Async Executor**: Submit kernel steps, apply MutationPlans under locks
//! - **Receipts/Audit**: Persist TransactionReceipts with policy snapshots
//! - **State Management**: CAS operations with per-key versioning and WAL

pub mod types;
pub mod zk_proof;

// Re-export types for convenience
pub use tokens::{AadhaarToken, MyNFTTokenData};
pub use types::{LedgerEntry, TokenValidationResult, TransactionLog, ZKProofMetadata};
pub use zk_proof::{execute_transaction, generate_zk_proof, save_proof_to_file, ProofResult};
