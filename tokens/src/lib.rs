#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod example_token;
pub mod nft_token;
pub mod aadhaar_token;
pub mod loantypes;
pub mod loan_token;
pub mod loan_pool_token;

pub use example_token::MyToken;
pub use nft_token::{MintInput, MyNFTTokenData, NFTToken, TransferInput};
pub use aadhaar_token::AadhaarToken;
pub use loan_token::{LoanToken, MintLoanTokenInput, UpdateLoanTokenInput};
pub use loan_pool_token::{LoanPoolToken, MintLoanPoolTokenInput, UpdateLoanPoolTokenInput};
