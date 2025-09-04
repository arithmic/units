extern crate alloc;

use super::loantypes::{PaymentDue, PoolPayout, Trustee};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use borsh::{BorshDeserialize, BorshSerialize};
use execution_engine::traits::TokenContract;
use execution_engine::types::{
    Address, ExecutionContext, KeyValue, TokenError, TokenResult, TransactionReceipt,
};
use execution_engine::utils::get_nonce_from_pre_state;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MintLoanPoolTokenInput {
    pub pool_name: String,
    pub pool_cut_off_date: String,
    pub status: String,
    pub broken_days: i32,
    pub pool_count: i32,
    pub pool_amount: f32,
    pub eis_method: String,
    pub finternet_originator_id: String,
    pub lender_share: f32,
    pub lender_os: f32,
    pub finternet_trustee_id: String,
    pub finternet_investor_id: String,
    pub investor_share: f32,
    pub investor_os: f32,
    pub payout_days: i32,
    pub fees_type: String,
    pub total_fees: f32,
    pub pool_xirr: f32,
    pub investor_fees: f32,
    pub direct_pricing: f32,
    pub pool_cycle: String,
    pub pool_health: String,
    pub interest_method: String,
    pub purchase_date: String,
    pub investor_repayment_type: String,
    pub loan: Vec<String>,
    pub payment_due: Vec<PaymentDue>,
    pub pool_payout: Vec<PoolPayout>,
    pub trustee: Trustee,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UpdateLoanPoolTokenInput {
    pub pool_name: String,
    pub pool_cut_off_date: Option<String>,
    pub status: Option<String>,
    pub broken_days: Option<i32>,
    pub pool_count: Option<i32>,
    pub pool_amount: Option<f32>,
    pub eis_method: Option<String>,
    pub finternet_originator_id: Option<String>,
    pub lender_share: Option<f32>,
    pub lender_os: Option<f32>,
    pub finternet_trustee_id: Option<String>,
    pub finternet_investor_id: Option<String>,
    pub investor_share: Option<f32>,
    pub investor_os: Option<f32>,
    pub payout_days: Option<i32>,
    pub fees_type: Option<String>,
    pub total_fees: Option<f32>,
    pub pool_xirr: Option<f32>,
    pub investor_fees: Option<f32>,
    pub direct_pricing: Option<f32>,
    pub pool_cycle: Option<String>,
    pub pool_health: Option<String>,
    pub interest_method: Option<String>,
    pub purchase_date: Option<String>,
    pub investor_repayment_type: Option<String>,
    pub loan: Option<Vec<String>>,
    pub payment_due: Option<Vec<PaymentDue>>,
    pub pool_payout: Option<Vec<PoolPayout>>,
    pub trustee: Option<Trustee>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct LoanPoolToken {
    pub pool_name: String,
    pub pool_cut_off_date: String,
    pub status: String,
    pub broken_days: i32,
    pub pool_count: i32,
    pub pool_amount: f32,
    pub eis_method: String,
    pub finternet_originator_id: String,
    pub lender_share: f32,
    pub lender_os: f32,
    pub finternet_trustee_id: String,
    pub finternet_investor_id: String,
    pub investor_share: f32,
    pub investor_os: f32,
    pub payout_days: i32,
    pub fees_type: String,
    pub total_fees: f32,
    pub pool_xirr: f32,
    pub investor_fees: f32,
    pub direct_pricing: f32,
    pub pool_cycle: String,
    pub pool_health: String,
    pub interest_method: String,
    pub purchase_date: String,
    pub investor_repayment_type: String,
    pub loan: Vec<String>,
    pub payment_due: Vec<PaymentDue>,
    pub pool_payout: Vec<PoolPayout>,
    pub trustee: Trustee,
}

impl LoanPoolToken {
    pub fn new(_admin_address: Address) -> Self {
        Self {
            pool_name: String::new(),
            pool_cut_off_date: String::new(),
            status: String::new(),
            broken_days: 0,
            pool_count: 0,
            pool_amount: 0.0,
            eis_method: String::new(),
            finternet_originator_id: String::new(),
            lender_share: 0.0,
            lender_os: 0.0,
            finternet_trustee_id: String::new(),
            finternet_investor_id: String::new(),
            investor_share: 0.0,
            investor_os: 0.0,
            payout_days: 0,
            fees_type: String::new(),
            total_fees: 0.0,
            pool_xirr: 0.0,
            investor_fees: 0.0,
            direct_pricing: 0.0,
            pool_cycle: String::new(),
            pool_health: String::new(),
            interest_method: String::new(),
            purchase_date: String::new(),
            investor_repayment_type: String::new(),
            loan: Vec::new(),
            payment_due: Vec::new(),
            pool_payout: Vec::new(),
            trustee: Trustee {
                company_name: String::new(),
                mobile_number: String::new(),
                email_id: String::new(),
                address_line1: String::new(),
                address_line2: String::new(),
                city: String::new(),
                state: String::new(),
                pincode: String::new(),
                country: String::new(),
            },
        }
    }

    pub fn mint(input: MintLoanPoolTokenInput) -> Self {
        Self {
            pool_name: input.pool_name,
            pool_cut_off_date: input.pool_cut_off_date,
            status: input.status,
            broken_days: input.broken_days,
            pool_count: input.pool_count,
            pool_amount: input.pool_amount,
            eis_method: input.eis_method,
            finternet_originator_id: input.finternet_originator_id,
            lender_share: input.lender_share,
            lender_os: input.lender_os,
            finternet_trustee_id: input.finternet_trustee_id,
            finternet_investor_id: input.finternet_investor_id,
            investor_share: input.investor_share,
            investor_os: input.investor_os,
            payout_days: input.payout_days,
            fees_type: input.fees_type,
            total_fees: input.total_fees,
            pool_xirr: input.pool_xirr,
            investor_fees: input.investor_fees,
            direct_pricing: input.direct_pricing,
            pool_cycle: input.pool_cycle,
            pool_health: input.pool_health,
            interest_method: input.interest_method,
            purchase_date: input.purchase_date,
            investor_repayment_type: input.investor_repayment_type,
            loan: input.loan,
            payment_due: input.payment_due,
            pool_payout: input.pool_payout,
            trustee: input.trustee,
        }
    }

    pub fn update(&mut self, input: UpdateLoanPoolTokenInput) {
        if input.pool_name != self.pool_name {
            // println!("warning: pool_name mismatch in update");
        }

        if let Some(pool_cut_off_date) = input.pool_cut_off_date {
            self.pool_cut_off_date = pool_cut_off_date;
        }
        if let Some(status) = input.status {
            self.status = status;
        }
        if let Some(broken_days) = input.broken_days {
            self.broken_days = broken_days;
        }
        if let Some(pool_count) = input.pool_count {
            self.pool_count = pool_count;
        }
        if let Some(pool_amount) = input.pool_amount {
            self.pool_amount = pool_amount;
        }
        if let Some(eis_method) = input.eis_method {
            self.eis_method = eis_method;
        }
        if let Some(finternet_originator_id) = input.finternet_originator_id {
            self.finternet_originator_id = finternet_originator_id;
        }
        if let Some(lender_share) = input.lender_share {
            self.lender_share = lender_share;
        }
        if let Some(lender_os) = input.lender_os {
            self.lender_os = lender_os;
        }
        if let Some(finternet_trustee_id) = input.finternet_trustee_id {
            self.finternet_trustee_id = finternet_trustee_id;
        }
        if let Some(finternet_investor_id) = input.finternet_investor_id {
            self.finternet_investor_id = finternet_investor_id;
        }
        if let Some(investor_share) = input.investor_share {
            self.investor_share = investor_share;
        }
        if let Some(investor_os) = input.investor_os {
            self.investor_os = investor_os;
        }
        if let Some(payout_days) = input.payout_days {
            self.payout_days = payout_days;
        }
        if let Some(fees_type) = input.fees_type {
            self.fees_type = fees_type;
        }
        if let Some(total_fees) = input.total_fees {
            self.total_fees = total_fees;
        }
        if let Some(pool_xirr) = input.pool_xirr {
            self.pool_xirr = pool_xirr;
        }
        if let Some(investor_fees) = input.investor_fees {
            self.investor_fees = investor_fees;
        }
        if let Some(direct_pricing) = input.direct_pricing {
            self.direct_pricing = direct_pricing;
        }
        if let Some(pool_cycle) = input.pool_cycle {
            self.pool_cycle = pool_cycle;
        }
        if let Some(pool_health) = input.pool_health {
            self.pool_health = pool_health;
        }
        if let Some(interest_method) = input.interest_method {
            self.interest_method = interest_method;
        }
        if let Some(purchase_date) = input.purchase_date {
            self.purchase_date = purchase_date;
        }
        if let Some(investor_repayment_type) = input.investor_repayment_type {
            self.investor_repayment_type = investor_repayment_type;
        }
        if let Some(loan) = input.loan {
            self.loan = loan;
        }
        if let Some(payment_due) = input.payment_due {
            self.payment_due = payment_due;
        }
        if let Some(pool_payout) = input.pool_payout {
            self.pool_payout = pool_payout;
        }
        if let Some(trustee) = input.trustee {
            self.trustee = trustee;
        }
    }

    fn mint_internal(
        &self,
        _ctx: &ExecutionContext,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let mint_input: MintLoanPoolTokenInput = borsh::from_slice(input)
            .map_err(|_| TokenError::Custom("failed to deserialize mint input".to_string()))?;

        let pool_key = self.get_pool_key(&mint_input.pool_name);
        let pool_token = LoanPoolToken::mint(mint_input);
        let pool_data = borsh::to_vec(&pool_token)
            .map_err(|_| TokenError::Custom("failed to serialize pool token".to_string()))?;

        let mut pool_bytes = [0u8; 32];
        let data_len = pool_data.len().min(32);
        pool_bytes[..data_len].copy_from_slice(&pool_data[..data_len]);

        let pool_write = KeyValue {
            key: pool_key,
            value: pool_bytes,
        };
        let mut writes = Vec::new();
        writes.push(pool_write);
        Ok(TransactionReceipt { writes })
    }

    fn update_internal(
        &self,
        _ctx: &ExecutionContext,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let update_input: UpdateLoanPoolTokenInput = borsh::from_slice(input)
            .map_err(|_| TokenError::Custom("failed to deserialize update input".to_string()))?;

        let pool_key = self.get_pool_key(&update_input.pool_name);

        // In a real implementation, we would read the existing pool from pre_state
        // For now, create a minimal update receipt
        let pool_write = KeyValue {
            key: pool_key,
            value: [1u8; 32], // Placeholder for updated pool data
        };

        let mut writes = Vec::new();
        writes.push(pool_write);
        Ok(TransactionReceipt { writes })
    }

    fn get_pool_key(&self, pool_name: &str) -> [u8; 32] {
        let mut key = [0u8; 32];
        let pool_bytes = pool_name.as_bytes();
        let copy_len = pool_bytes.len().min(32);
        key[..copy_len].copy_from_slice(&pool_bytes[..copy_len]);
        key
    }
}

impl TokenContract for LoanPoolToken {
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let current_nonce = get_nonce_from_pre_state(ctx.signer, ctx.pre_state);
        if ctx.nonce != current_nonce {
            return Err(TokenError::InvalidNonce);
        }

        let receipt = match function {
            "mint" => self.mint_internal(ctx, input),
            "update" => self.update_internal(ctx, input),
            _ => Err(TokenError::FunctionNotFound),
        }?;
        Ok(receipt)
    }
}
