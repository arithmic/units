use crate::{traits::Hasher, H256};

#[cfg(feature = "blake2b-rs")]
use blake2b_rs::{Blake2b, Blake2bBuilder};

#[cfg(feature = "blake2b-rs")]
const BLAKE2B_KEY: &[u8] = &[];
#[cfg(feature = "blake2b-rs")]
const BLAKE2B_LEN: usize = 32;
#[cfg(feature = "blake2b-rs")]
const PERSONALIZATION: &[u8] = b"sparsemerkletree";

#[cfg(feature = "blake2b-rs")]
pub struct Blake2bHasher(Blake2b);

#[cfg(feature = "blake2b-rs")]
impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2bBuilder::new(BLAKE2B_LEN)
            .personal(PERSONALIZATION)
            .key(BLAKE2B_KEY)
            .build();
        Blake2bHasher(blake2b)
    }
}

#[cfg(feature = "blake2b-rs")]
impl Hasher for Blake2bHasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice());
    }
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }
    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        self.0.finalize(&mut hash);
        hash.into()
    }
}

#[cfg(not(feature = "blake2b-rs"))]
pub struct Blake2bHasher;

#[cfg(not(feature = "blake2b-rs"))]
impl Default for Blake2bHasher {
    fn default() -> Self {
        Blake2bHasher
    }
}

#[cfg(not(feature = "blake2b-rs"))]
impl Hasher for Blake2bHasher {
    fn write_h256(&mut self, _h: &H256) {
        // Blake2b not available when feature is disabled
    }
    fn write_byte(&mut self, _b: u8) {
        // Blake2b not available when feature is disabled
    }
    fn finish(self) -> H256 {
        // Return zero hash when Blake2b is not available
        H256::zero()
    }
}
