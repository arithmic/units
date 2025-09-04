use crate::*;
use sha2::{Sha256, Digest};
use default_store::DefaultStore;
use proptest::prelude::*;
use traits::Hasher;

pub struct TestSha256Hasher(Sha256);

impl Default for TestSha256Hasher {
    fn default() -> Self {
        TestSha256Hasher(Sha256::new())
    }
}

impl Hasher for TestSha256Hasher {
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b]);
    }
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }
    fn finish(self) -> H256 {
        let result = self.0.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash.into()
    }
}

pub type TestSMT = SparseMerkleTree<TestSha256Hasher, H256, DefaultStore<H256>>;

proptest! {
    #[test]
    fn test_random_merkle_proof(key: [u8; 32], value: [u8;32]) {
        let key = H256::from(key);
        let value = H256::from(value);
        const EXPECTED_PROOF_SIZE: usize = 16;

        let mut tree = TestSMT::default();
        tree.update(key, value).expect("update");

        let proof = tree.merkle_proof(vec![key]).expect("gen proof");
        let compiled_proof = proof.clone().compile(vec![key]).expect("compile proof");
        assert!(proof
            .verify::<TestSha256Hasher>(tree.root(), vec![(key, value)])
            .expect("verify"));
        assert!(compiled_proof
            .verify::<TestSha256Hasher>(tree.root(), vec![(key, value)])
            .expect("verify compiled proof"));

        let single_compiled_proof = compiled_proof
            .extract_proof::<TestSha256Hasher>(vec![(key, value, true)])
            .expect("compiled one proof");
        assert!(single_compiled_proof
            .verify::<TestSha256Hasher>(tree.root(), vec![(key, value)])
            .expect("verify compiled one proof"));
    }
}