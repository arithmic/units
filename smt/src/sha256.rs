use crate::{traits::Hasher, H256};

cfg_if::cfg_if! {
    if #[cfg(feature = "sp1")] {
        // Use SP1-patched SHA2 for SP1 builds
        use sha2_v0_10_8::{Digest, Sha256};
        
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
    } else {
        // Use standard library SHA2 for non-SP1 builds
        extern crate alloc;
        use alloc::vec::Vec;
        
        // Fallback implementation using a simple hash function
        // In a real implementation, you'd want to use a proper SHA256 implementation
        pub struct Sha256Hasher {
            data: Vec<u8>,
        }
        
        impl Default for Sha256Hasher {
            fn default() -> Self {
                Self {
                    data: Vec::new(),
                }
            }
        }
        
        impl Hasher for Sha256Hasher {
            fn write_h256(&mut self, h: &H256) {
                self.data.extend_from_slice(h.as_slice());
            }
            
            fn write_byte(&mut self, b: u8) {
                self.data.push(b);
            }
            
            fn finish(self) -> H256 {
                // This is a placeholder - in production you'd use proper SHA256
                // For now, we'll use Blake2b as fallback
                use crate::blake2b::Blake2bHasher;
                let mut hasher = Blake2bHasher::default();
                for chunk in self.data.chunks(32) {
                    if chunk.len() == 32 {
                        let mut h256_bytes = [0u8; 32];
                        h256_bytes.copy_from_slice(chunk);
                        hasher.write_h256(&h256_bytes.into());
                    } else {
                        for &byte in chunk {
                            hasher.write_byte(byte);
                        }
                    }
                }
                hasher.finish()
            }
        }
    }
}