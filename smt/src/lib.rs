//! Constructs a new `SparseMerkleTree<H, V, S>`.
//!
//! # Examples
//!
//! ```
//! use sparse_merkle_tree::{
//!     default_store::DefaultStore, error::Error, MerkleProof,
//!     sha256::Sha256Hasher, SparseMerkleTree, traits::Value, H256
//! };
//!
//! // define SMT
//! type SMT = SparseMerkleTree<Sha256Hasher, Word, DefaultStore<Word>>;
//!
//! // define SMT value
//! #[derive(Default, Clone)]
//! pub struct Word(String);
//! impl Value for Word {
//!    fn to_h256(&self) -> H256 {
//!        if self.0.is_empty() {
//!            return H256::zero();
//!        }
//!        use sha2::{Sha256, Digest};
//!        let mut hasher = Sha256::new();
//!        hasher.update(self.0.as_bytes());
//!        let result = hasher.finalize();
//!        let mut hash = [0u8; 32];
//!        hash.copy_from_slice(&result);
//!        hash.into()
//!    }
//!    fn zero() -> Self {
//!        Default::default()
//!    }
//! }
//!
//! fn construct_smt() {
//!     let mut tree = SMT::default();
//!     for (i, word) in "The quick brown fox jumps over the lazy dog"
//!         .split_whitespace()
//!         .enumerate()
//!     {
//!         let key: H256 = {
//!             use sha2::{Sha256, Digest};
//!             let mut hasher = Sha256::new();
//!             hasher.update(&(i as u32).to_le_bytes());
//!             let result = hasher.finalize();
//!             let mut hash = [0u8; 32];
//!             hash.copy_from_slice(&result);
//!             hash.into()
//!         };
//!         let value = Word(word.to_string());
//!         // insert key value into tree
//!         tree.update(key, value).expect("update");
//!     }
//!
//!     println!("SMT root is {:?} ", tree.root());
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_store;
pub mod error;
pub mod h256;
pub mod merge;
pub mod merkle_proof;
pub mod sha256;
#[cfg(test)]
mod tests;
pub mod traits;
mod tree;

pub use h256::H256;
pub use merkle_proof::{CompiledMerkleProof, MerkleProof};
pub use tree::SparseMerkleTree;
pub use tree::{BranchKey, BranchNode};

/// Expected path size: log2(256) * 2, used for hint vector capacity
pub const EXPECTED_PATH_SIZE: usize = 16;
// Max stack size can be used when verify compiled proof
pub(crate) const MAX_STACK_SIZE: usize = 257;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::collections;
        use std::vec;
        use std::string;
    } else {
        extern crate alloc;
        use alloc::collections;
        use alloc::vec;
        use alloc::string;
    }
}
