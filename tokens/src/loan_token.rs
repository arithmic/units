extern crate alloc;

use super::loantypes::{
    Applicant, Charge, InvestorCharge, InvestorRps, LenderCharge, OriginatorRps, Repayment,
    Transaction,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use borsh::{BorshDeserialize, BorshSerialize};
use execution_engine::traits::TokenContract;
use execution_engine::types::{
    Address, ExecutionContext, KeyValue, TokenError, TokenResult, TransactionReceipt,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MintLoanTokenInput {
    pub loan_account_number: String,
    pub sanction_amount: f32,
    pub loan_amount: f32,
    pub disbursement_amount: f32,
    pub interest_rate: f32,
    pub tenure: i32,
    pub investor_os: f32,
    pub cersai_date: String,
    pub loan_rescheduled: i32,
    pub cersai_registration_number: String,
    pub sourcing_branch: String,
    pub loan_type: String,
    pub program: String,
    pub status: String,
    pub product_type: String,
    pub transaction_type: String,
    pub product_code: String,
    pub total_income: f32,
    pub total_obligation: f32,
    pub max_dpd: i32,
    pub loan_restructured: i32,
    pub sourcing_state: String,
    pub emi_day: i32,
    pub emi_per_month: f32,
    pub disbursement_date: String,
    pub roi_type: String,
    pub last_12_months_bounce_count: String,
    pub emi_paid: f32,
    pub balance: f32,
    pub emi_bounced: String,
    pub property_type: String,
    pub property_status: String,
    pub collateral_location: String,
    pub ltv: String,
    pub property_value: f32,
    pub foir: f32,
    pub credit_score: i32,
    pub outstanding_balance: f32,
    pub repayment_schedule: Vec<Repayment>,
    pub applicant: Vec<Applicant>,
    pub transaction: Vec<Transaction>,
    pub charges: Vec<Charge>,
    pub investor_rps: Vec<InvestorRps>,
    pub originator_rps: Vec<OriginatorRps>,
    pub investor_charges: Vec<InvestorCharge>,
    pub lender_charges: Vec<LenderCharge>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UpdateLoanTokenInput {
    pub loan_account_number: String,
    pub sanction_amount: Option<f32>,
    pub loan_amount: Option<f32>,
    pub disbursement_amount: Option<f32>,
    pub interest_rate: Option<f32>,
    pub tenure: Option<i32>,
    pub investor_os: Option<f32>,
    pub cersai_date: Option<String>,
    pub loan_rescheduled: Option<i32>,
    pub cersai_registration_number: Option<String>,
    pub sourcing_branch: Option<String>,
    pub loan_type: Option<String>,
    pub program: Option<String>,
    pub status: Option<String>,
    pub product_type: Option<String>,
    pub transaction_type: Option<String>,
    pub product_code: Option<String>,
    pub total_income: Option<f32>,
    pub total_obligation: Option<f32>,
    pub max_dpd: Option<i32>,
    pub loan_restructured: Option<i32>,
    pub sourcing_state: Option<String>,
    pub emi_day: Option<i32>,
    pub emi_per_month: Option<f32>,
    pub disbursement_date: Option<String>,
    pub roi_type: Option<String>,
    pub last_12_months_bounce_count: Option<String>,
    pub emi_paid: Option<f32>,
    pub balance: Option<f32>,
    pub emi_bounced: Option<String>,
    pub property_type: Option<String>,
    pub property_status: Option<String>,
    pub collateral_location: Option<String>,
    pub ltv: Option<String>,
    pub property_value: Option<f32>,
    pub foir: Option<f32>,
    pub credit_score: Option<i32>,
    pub outstanding_balance: Option<f32>,
    pub repayment_schedule: Option<Vec<Repayment>>,
    pub applicant: Option<Vec<Applicant>>,
    pub transaction: Option<Vec<Transaction>>,
    pub charges: Option<Vec<Charge>>,
    pub investor_rps: Option<Vec<InvestorRps>>,
    pub originator_rps: Option<Vec<OriginatorRps>>,
    pub investor_charges: Option<Vec<InvestorCharge>>,
    pub lender_charges: Option<Vec<LenderCharge>>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct LoanToken {
    pub loan_account_number: String,
    pub sanction_amount: f32,
    pub loan_amount: f32,
    pub disbursement_amount: f32,
    pub interest_rate: f32,
    pub tenure: i32,
    pub investor_os: f32,
    pub cersai_date: String,
    pub loan_rescheduled: i32,
    pub cersai_registration_number: String,
    pub sourcing_branch: String,
    pub loan_type: String,
    pub program: String,
    pub status: String,
    pub product_type: String,
    pub transaction_type: String,
    pub product_code: String,
    pub total_income: f32,
    pub total_obligation: f32,
    pub max_dpd: i32,
    pub loan_restructured: i32,
    pub sourcing_state: String,
    pub emi_day: i32,
    pub emi_per_month: f32,
    pub disbursement_date: String,
    pub roi_type: String,
    pub last_12_months_bounce_count: String,
    pub emi_paid: f32,
    pub balance: f32,
    pub emi_bounced: String,
    pub property_type: String,
    pub property_status: String,
    pub collateral_location: String,
    pub ltv: String,
    pub property_value: f32,
    pub foir: f32,
    pub credit_score: i32,
    pub outstanding_balance: f32,
    pub repayment_schedule: Vec<Repayment>,
    pub applicant: Vec<Applicant>,
    pub transaction: Vec<Transaction>,
    pub charges: Vec<Charge>,
    pub investor_rps: Vec<InvestorRps>,
    pub originator_rps: Vec<OriginatorRps>,
    pub investor_charges: Vec<InvestorCharge>,
    pub lender_charges: Vec<LenderCharge>,
}

impl LoanToken {
    pub fn new(_admin_address: Address) -> Self {
        Self {
            loan_account_number: String::new(),
            sanction_amount: 0.0,
            loan_amount: 0.0,
            disbursement_amount: 0.0,
            interest_rate: 0.0,
            tenure: 0,
            investor_os: 0.0,
            cersai_date: String::new(),
            loan_rescheduled: 0,
            cersai_registration_number: String::new(),
            sourcing_branch: String::new(),
            loan_type: String::new(),
            program: String::new(),
            status: String::new(),
            product_type: String::new(),
            transaction_type: String::new(),
            product_code: String::new(),
            total_income: 0.0,
            total_obligation: 0.0,
            max_dpd: 0,
            loan_restructured: 0,
            sourcing_state: String::new(),
            emi_day: 0,
            emi_per_month: 0.0,
            disbursement_date: String::new(),
            roi_type: String::new(),
            last_12_months_bounce_count: String::new(),
            emi_paid: 0.0,
            balance: 0.0,
            emi_bounced: String::new(),
            property_type: String::new(),
            property_status: String::new(),
            collateral_location: String::new(),
            ltv: String::new(),
            property_value: 0.0,
            foir: 0.0,
            credit_score: 0,
            outstanding_balance: 0.0,
            repayment_schedule: Vec::new(),
            applicant: Vec::new(),
            transaction: Vec::new(),
            charges: Vec::new(),
            investor_rps: Vec::new(),
            originator_rps: Vec::new(),
            investor_charges: Vec::new(),
            lender_charges: Vec::new(),
        }
    }

    pub fn mint(input: MintLoanTokenInput) -> Self {
        Self {
            loan_account_number: input.loan_account_number,
            sanction_amount: input.sanction_amount,
            loan_amount: input.loan_amount,
            disbursement_amount: input.disbursement_amount,
            interest_rate: input.interest_rate,
            tenure: input.tenure,
            investor_os: input.investor_os,
            cersai_date: input.cersai_date,
            loan_rescheduled: input.loan_rescheduled,
            cersai_registration_number: input.cersai_registration_number,
            sourcing_branch: input.sourcing_branch,
            loan_type: input.loan_type,
            program: input.program,
            status: input.status,
            product_type: input.product_type,
            transaction_type: input.transaction_type,
            product_code: input.product_code,
            total_income: input.total_income,
            total_obligation: input.total_obligation,
            max_dpd: input.max_dpd,
            loan_restructured: input.loan_restructured,
            sourcing_state: input.sourcing_state,
            emi_day: input.emi_day,
            emi_per_month: input.emi_per_month,
            disbursement_date: input.disbursement_date,
            roi_type: input.roi_type,
            last_12_months_bounce_count: input.last_12_months_bounce_count,
            emi_paid: input.emi_paid,
            balance: input.balance,
            emi_bounced: input.emi_bounced,
            property_type: input.property_type,
            property_status: input.property_status,
            collateral_location: input.collateral_location,
            ltv: input.ltv,
            property_value: input.property_value,
            foir: input.foir,
            credit_score: input.credit_score,
            outstanding_balance: input.outstanding_balance,
            repayment_schedule: input.repayment_schedule,
            applicant: input.applicant,
            transaction: input.transaction,
            charges: input.charges,
            investor_rps: input.investor_rps,
            originator_rps: input.originator_rps,
            investor_charges: input.investor_charges,
            lender_charges: input.lender_charges,
        }
    }

    pub fn update(&mut self, input: UpdateLoanTokenInput) {
        if input.loan_account_number != self.loan_account_number {
            // println!("warning: loan_account_number mismatch in update");
        }

        if let Some(sanction_amount) = input.sanction_amount {
            self.sanction_amount = sanction_amount;
        }
        if let Some(loan_amount) = input.loan_amount {
            self.loan_amount = loan_amount;
        }
        if let Some(disbursement_amount) = input.disbursement_amount {
            self.disbursement_amount = disbursement_amount;
        }
        if let Some(interest_rate) = input.interest_rate {
            self.interest_rate = interest_rate;
        }
        if let Some(tenure) = input.tenure {
            self.tenure = tenure;
        }
        if let Some(investor_os) = input.investor_os {
            self.investor_os = investor_os;
        }
        if let Some(cersai_date) = input.cersai_date {
            self.cersai_date = cersai_date;
        }
        if let Some(loan_rescheduled) = input.loan_rescheduled {
            self.loan_rescheduled = loan_rescheduled;
        }
        if let Some(cersai_registration_number) = input.cersai_registration_number {
            self.cersai_registration_number = cersai_registration_number;
        }
        if let Some(sourcing_branch) = input.sourcing_branch {
            self.sourcing_branch = sourcing_branch;
        }
        if let Some(loan_type) = input.loan_type {
            self.loan_type = loan_type;
        }
        if let Some(program) = input.program {
            self.program = program;
        }
        if let Some(status) = input.status {
            self.status = status;
        }
        if let Some(product_type) = input.product_type {
            self.product_type = product_type;
        }
        if let Some(transaction_type) = input.transaction_type {
            self.transaction_type = transaction_type;
        }
        if let Some(product_code) = input.product_code {
            self.product_code = product_code;
        }
        if let Some(total_income) = input.total_income {
            self.total_income = total_income;
        }
        if let Some(total_obligation) = input.total_obligation {
            self.total_obligation = total_obligation;
        }
        if let Some(max_dpd) = input.max_dpd {
            self.max_dpd = max_dpd;
        }
        if let Some(loan_restructured) = input.loan_restructured {
            self.loan_restructured = loan_restructured;
        }
        if let Some(sourcing_state) = input.sourcing_state {
            self.sourcing_state = sourcing_state;
        }
        if let Some(emi_day) = input.emi_day {
            self.emi_day = emi_day;
        }
        if let Some(emi_per_month) = input.emi_per_month {
            self.emi_per_month = emi_per_month;
        }
        if let Some(disbursement_date) = input.disbursement_date {
            self.disbursement_date = disbursement_date;
        }
        if let Some(roi_type) = input.roi_type {
            self.roi_type = roi_type;
        }
        if let Some(last_12_months_bounce_count) = input.last_12_months_bounce_count {
            self.last_12_months_bounce_count = last_12_months_bounce_count;
        }
        if let Some(emi_paid) = input.emi_paid {
            self.emi_paid = emi_paid;
        }
        if let Some(balance) = input.balance {
            self.balance = balance;
        }
        if let Some(emi_bounced) = input.emi_bounced {
            self.emi_bounced = emi_bounced;
        }
        if let Some(property_type) = input.property_type {
            self.property_type = property_type;
        }
        if let Some(property_status) = input.property_status {
            self.property_status = property_status;
        }
        if let Some(collateral_location) = input.collateral_location {
            self.collateral_location = collateral_location;
        }
        if let Some(ltv) = input.ltv {
            self.ltv = ltv;
        }
        if let Some(property_value) = input.property_value {
            self.property_value = property_value;
        }
        if let Some(foir) = input.foir {
            self.foir = foir;
        }
        if let Some(credit_score) = input.credit_score {
            self.credit_score = credit_score;
        }
        if let Some(outstanding_balance) = input.outstanding_balance {
            self.outstanding_balance = outstanding_balance;
        }
        if let Some(repayment_schedule) = input.repayment_schedule {
            self.repayment_schedule = repayment_schedule;
        }
        if let Some(applicant) = input.applicant {
            self.applicant = applicant;
        }
        if let Some(transaction) = input.transaction {
            self.transaction = transaction;
        }
        if let Some(charges) = input.charges {
            self.charges = charges;
        }
        if let Some(investor_rps) = input.investor_rps {
            self.investor_rps = investor_rps;
        }
        if let Some(originator_rps) = input.originator_rps {
            self.originator_rps = originator_rps;
        }
        if let Some(investor_charges) = input.investor_charges {
            self.investor_charges = investor_charges;
        }
        if let Some(lender_charges) = input.lender_charges {
            self.lender_charges = lender_charges;
        }
    }

    fn mint_internal(
        &self,
        _ctx: &ExecutionContext,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let mint_input: MintLoanTokenInput = borsh::from_slice(input)
            .map_err(|_| TokenError::Custom("failed to deserialize mint input".to_string()))?;

        let loan_key = self.get_loan_key(&mint_input.loan_account_number);
        let loan_token = LoanToken::mint(mint_input);
        let loan_data = borsh::to_vec(&loan_token)
            .map_err(|_| TokenError::Custom("failed to serialize loan token".to_string()))?;

        let mut loan_bytes = [0u8; 32];
        let data_len = loan_data.len().min(32);
        loan_bytes[..data_len].copy_from_slice(&loan_data[..data_len]);

        let loan_write = KeyValue {
            key: loan_key,
            value: loan_bytes,
        };

        Ok(TransactionReceipt {
            writes: vec![loan_write],
        })
    }

    fn update_internal(
        &self,
        _ctx: &ExecutionContext,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let update_input: UpdateLoanTokenInput = borsh::from_slice(input)
            .map_err(|_| TokenError::Custom("failed to deserialize update input".to_string()))?;

        let loan_key = self.get_loan_key(&update_input.loan_account_number);

        // In a real implementation, we would read the existing loan from pre_state
        // For now, create a minimal update receipt
        let loan_write = KeyValue {
            key: loan_key,
            value: [1u8; 32], // Placeholder for updated loan data
        };

        Ok(TransactionReceipt {
            writes: vec![loan_write],
        })
    }

    fn get_loan_key(&self, loan_account_number: &str) -> [u8; 32] {
        let mut key = [0u8; 32];
        let account_bytes = loan_account_number.as_bytes();
        let copy_len = account_bytes.len().min(32);
        key[..copy_len].copy_from_slice(&account_bytes[..copy_len]);
        key
    }
}

impl TokenContract for LoanToken {
    fn execute(
        &self,
        ctx: &ExecutionContext,
        function: &str,
        input: &[u8],
    ) -> TokenResult<TransactionReceipt> {
        let receipt = match function {
            "mint" => self.mint_internal(ctx, input),
            "update" => self.update_internal(ctx, input),
            _ => Err(TokenError::FunctionNotFound),
        }?;

        Ok(receipt)
    }
}
