use crate::{traits::Hasher, H256};

// Always use SHA2 crate which supports no_std
use sha2::{Digest, Sha256};

pub struct Sha256Hasher {
    hasher: Sha256,
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }
}

impl Hasher for Sha256Hasher {
    fn write_h256(&mut self, h: &H256) {
        self.hasher.update(h.as_slice());
    }
    
    fn write_byte(&mut self, b: u8) {
        self.hasher.update(&[b]);
    }
    
    fn finish(self) -> H256 {
        let result = self.hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash.into()
    }
}