#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod example_token;
pub mod nft_token;
pub mod aadhaar_token;

pub use example_token::MyToken;
pub use nft_token::{MintInput, MyNFTTokenData, NFTToken, TransferInput};
pub use aadhaar_token::AadhaarToken;
