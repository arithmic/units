//! Comprehensive example showcasing sparse merkle tree usage
//!
//! This example demonstrates various operations and use cases for the sparse merkle tree,
//! including basic operations, custom value types, merkle proofs, and batch operations.

use sparse_merkle_tree::{
    default_store::DefaultStore, error::Error, sha256::Sha256Hasher, traits::Value,
    SparseMerkleTree, H256,
};

use std::collections::HashMap;

/// Custom value type representing a user account
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
    pub username: String,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            balance: 0,
            nonce: 0,
            username: String::new(),
        }
    }
}

impl Value for Account {
    fn to_h256(&self) -> H256 {
        if self.balance == 0 && self.nonce == 0 && self.username.is_empty() {
            return H256::zero();
        }

        // Use a simple hash combining all fields
        let mut data = Vec::new();
        data.extend_from_slice(&self.balance.to_le_bytes());
        data.extend_from_slice(&self.nonce.to_le_bytes());
        data.extend_from_slice(self.username.as_bytes());

        // Create a deterministic hash from the data
        let mut hash_bytes = [0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        // Add some more mixing
        for i in 0..32 {
            hash_bytes[i] = hash_bytes[i].wrapping_add((i as u8).wrapping_mul(17));
        }
        H256::from(hash_bytes)
    }

    fn zero() -> Self {
        Default::default()
    }
}

/// Custom value type for simple string storage
#[derive(Debug, Clone, PartialEq)]
pub struct Word(pub String);

impl Default for Word {
    fn default() -> Self {
        Word(String::new())
    }
}

impl Value for Word {
    fn to_h256(&self) -> H256 {
        if self.0.is_empty() {
            return H256::zero();
        }

        // Create a simple hash from the string
        let mut hash_bytes = [0u8; 32];
        let bytes = self.0.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        // Add some mixing to make it more deterministic
        for i in 0..32 {
            hash_bytes[i] = hash_bytes[i].wrapping_add((i as u8).wrapping_mul(23));
        }
        H256::from(hash_bytes)
    }

    fn zero() -> Self {
        Default::default()
    }
}

type AccountSMT = SparseMerkleTree<Sha256Hasher, Account, DefaultStore<Account>>;
type WordSMT = SparseMerkleTree<Sha256Hasher, Word, DefaultStore<Word>>;
type SimpleSMT = SparseMerkleTree<Sha256Hasher, H256, DefaultStore<H256>>;

/// Example 1: Basic SMT operations with H256 values
fn example_basic_operations() -> Result<(), Error> {
    println!("=== Example 1: Basic SMT Operations ===");

    let mut tree = SimpleSMT::default();
    println!("Initial root: {:?}", tree.root());
    println!("Is empty: {}", tree.is_empty());

    // Create some test keys and values using simple hashing
    let key1: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"key1";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    let key2: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"key2";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    let value1: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"value1";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    let value2: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"value2";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    // Insert first key-value pair
    tree.update(key1, value1)?;
    println!("After inserting key1: root = {:?}", tree.root());
    println!("Is empty: {}", tree.is_empty());

    // Insert second key-value pair
    tree.update(key2, value2)?;
    println!("After inserting key2: root = {:?}", tree.root());

    // Retrieve values
    let retrieved1 = tree.get(&key1)?;
    let retrieved2 = tree.get(&key2)?;
    println!("Retrieved value1: {:?}", retrieved1);
    println!("Retrieved value2: {:?}", retrieved2);

    // Try to get a non-existent key
    let non_existent_key: H256 = [1u8; 32].into();
    let non_existent_value = tree.get(&non_existent_key)?;
    println!("Non-existent key returns: {:?}", non_existent_value);

    // Delete a key by setting it to zero
    tree.update(key1, H256::zero())?;
    println!("After deleting key1: root = {:?}", tree.root());
    let deleted_value = tree.get(&key1)?;
    println!("Deleted key1 now returns: {:?}", deleted_value);

    Ok(())
}

/// Example 2: Using custom value types (Account management)
fn example_custom_account_type() -> Result<(), Error> {
    println!("\n=== Example 2: Custom Account Type ===");

    let mut tree = AccountSMT::default();

    // Create user accounts
    let alice_key: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"alice";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    let bob_key: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"bob";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    let alice_account = Account {
        balance: 1000,
        nonce: 1,
        username: "alice".to_string(),
    };

    let bob_account = Account {
        balance: 500,
        nonce: 0,
        username: "bob".to_string(),
    };

    // Insert accounts
    tree.update(alice_key, alice_account.clone())?;
    tree.update(bob_key, bob_account.clone())?;

    println!("Tree root after adding accounts: {:?}", tree.root());

    // Retrieve accounts
    let retrieved_alice = tree.get(&alice_key)?;
    let retrieved_bob = tree.get(&bob_key)?;

    println!("Alice's account: {:?}", retrieved_alice);
    println!("Bob's account: {:?}", retrieved_bob);

    // Update Alice's balance (simulate a transaction)
    let updated_alice = Account {
        balance: 800,
        nonce: 2,
        username: "alice".to_string(),
    };

    tree.update(alice_key, updated_alice)?;
    println!("Tree root after updating Alice: {:?}", tree.root());

    let updated_retrieved = tree.get(&alice_key)?;
    println!("Alice's updated account: {:?}", updated_retrieved);

    Ok(())
}

/// Example 3: Word storage with string values
fn example_word_storage() -> Result<(), Error> {
    println!("\n=== Example 3: Word Storage ===");

    let mut tree = WordSMT::default();

    let sentence = "The quick brown fox jumps over the lazy dog";
    let mut keys = Vec::new();

    // Insert each word with an index-based key
    for (i, word) in sentence.split_whitespace().enumerate() {
        let key: H256 = {
            let mut hash_bytes = [0u8; 32];
            let data = (i as u32).to_le_bytes();
            for (j, &byte) in data.iter().enumerate() {
                hash_bytes[j % 32] ^= byte;
            }
            H256::from(hash_bytes)
        };

        let value = Word(word.to_string());
        tree.update(key, value)?;
        keys.push(key);

        println!("Inserted word '{}' at index {}", word, i);
    }

    println!("Final tree root: {:?}", tree.root());

    // Retrieve and display all words
    println!("Retrieved words:");
    for (i, key) in keys.iter().enumerate() {
        let word = tree.get(key)?;
        println!("Index {}: '{}'", i, word.0);
    }

    Ok(())
}

/// Example 4: Batch operations
fn example_batch_operations() -> Result<(), Error> {
    println!("\n=== Example 4: Batch Operations ===");

    let mut tree = SimpleSMT::default();

    // Prepare multiple key-value pairs
    let mut pairs = Vec::new();
    for i in 0..10 {
        let key: H256 = {
            let mut hash_bytes = [0u8; 32];
            let data = (i as u32).to_le_bytes();
            for (j, &byte) in data.iter().enumerate() {
                hash_bytes[j % 32] ^= byte;
            }
            H256::from(hash_bytes)
        };

        let value: H256 = {
            let mut hash_bytes = [0u8; 32];
            let data = ((i * 10) as u32).to_le_bytes();
            for (j, &byte) in data.iter().enumerate() {
                hash_bytes[j % 32] ^= byte;
            }
            H256::from(hash_bytes)
        };

        pairs.push((key, value));
    }

    println!("Performing batch update of {} items", pairs.len());

    // Batch update
    tree.update_all(pairs.clone())?;
    println!("Batch update completed. Root: {:?}", tree.root());

    // Verify all values were inserted correctly
    println!("Verifying inserted values:");
    for (i, (key, expected_value)) in pairs.iter().enumerate() {
        let retrieved = tree.get(key)?;
        println!(
            "Key {}: expected = {:?}, retrieved = {:?}, match = {}",
            i,
            expected_value,
            retrieved,
            expected_value == &retrieved
        );
    }

    Ok(())
}

/// Example 5: Merkle proofs
fn example_merkle_proofs() -> Result<(), Error> {
    println!("\n=== Example 5: Merkle Proofs ===");

    let mut tree = SimpleSMT::default();

    // Insert some test data
    let mut test_data = HashMap::new();
    for i in 0..5 {
        let key: H256 = {
            let mut hash_bytes = [0u8; 32];
            let data = format!("test_key_{}", i);
            for (j, &byte) in data.as_bytes().iter().enumerate() {
                hash_bytes[j % 32] ^= byte;
            }
            H256::from(hash_bytes)
        };

        let value: H256 = {
            let mut hash_bytes = [0u8; 32];
            let data = format!("test_value_{}", i);
            for (j, &byte) in data.as_bytes().iter().enumerate() {
                hash_bytes[j % 32] ^= byte;
            }
            H256::from(hash_bytes)
        };

        tree.update(key, value)?;
        test_data.insert(key, value);
    }

    let tree_root = *tree.root();
    println!("Tree root: {:?}", tree_root);

    // Generate proof for the first key
    let first_key = test_data.keys().next().unwrap();
    let first_value = test_data[first_key];

    let proof_keys = vec![*first_key];
    let merkle_proof = tree.merkle_proof(proof_keys)?;

    println!("Generated merkle proof for key: {:?}", first_key);
    println!("Proof size: {} items", merkle_proof.merkle_path().len());

    // Verify the proof
    let proof_items = vec![(*first_key, first_value)];
    let computed_root = merkle_proof
        .clone()
        .compute_root::<Sha256Hasher>(proof_items)?;

    println!("Original root:  {:?}", tree_root);
    println!("Computed root:  {:?}", computed_root);
    println!("Proof valid: {}", tree_root == computed_root);

    // Test proof with wrong value (should fail)
    let wrong_value: H256 = [255u8; 32].into();
    let wrong_proof_items = vec![(*first_key, wrong_value)];
    let wrong_computed_root = merkle_proof.compute_root::<Sha256Hasher>(wrong_proof_items)?;

    println!("Wrong computed root: {:?}", wrong_computed_root);
    println!("Wrong proof valid: {}", tree_root == wrong_computed_root);

    Ok(())
}

/// Example 6: Tree persistence and reconstruction
fn example_persistence() -> Result<(), Error> {
    println!("\n=== Example 6: Tree Persistence ===");

    // Create and populate a tree
    let mut original_tree = SimpleSMT::default();

    let key: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"persistent_key";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    let value: H256 = {
        let mut hash_bytes = [0u8; 32];
        let data = b"persistent_value";
        for (i, &byte) in data.iter().enumerate() {
            hash_bytes[i % 32] ^= byte;
        }
        H256::from(hash_bytes)
    };

    original_tree.update(key, value)?;
    let original_root = *original_tree.root();

    println!("Original tree root: {:?}", original_root);

    // Extract the store
    let store = original_tree.take_store();

    // Reconstruct tree from store
    let reconstructed_tree = SimpleSMT::new_with_store(store)?;
    let reconstructed_root = *reconstructed_tree.root();

    println!("Reconstructed tree root: {:?}", reconstructed_root);
    println!("Roots match: {}", original_root == reconstructed_root);

    // Verify data integrity
    let retrieved_value = reconstructed_tree.get(&key)?;
    println!("Retrieved value matches: {}", value == retrieved_value);

    Ok(())
}

fn main() -> Result<(), Error> {
    println!("Sparse Merkle Tree Usage Examples");
    println!("==================================");

    example_basic_operations()?;
    example_custom_account_type()?;
    example_word_storage()?;
    example_batch_operations()?;
    example_merkle_proofs()?;
    example_persistence()?;

    println!("\n=== All Examples Completed Successfully! ===");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert!(example_basic_operations().is_ok());
        assert!(example_custom_account_type().is_ok());
        assert!(example_word_storage().is_ok());
        assert!(example_batch_operations().is_ok());
        assert!(example_merkle_proofs().is_ok());
        assert!(example_persistence().is_ok());
    }

    #[test]
    fn test_account_value_trait() {
        let account = Account {
            balance: 100,
            nonce: 1,
            username: "test".to_string(),
        };

        let hash = account.to_h256();
        assert_ne!(hash, H256::zero());

        let zero_account = Account::zero();
        assert_eq!(zero_account.to_h256(), H256::zero());
    }

    #[test]
    fn test_word_value_trait() {
        let word = Word("hello".to_string());
        let hash = word.to_h256();
        assert_ne!(hash, H256::zero());

        let empty_word = Word::zero();
        assert_eq!(empty_word.to_h256(), H256::zero());
    }
}
