extern crate alloc;

use alloc::string::String;
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Repayment {
    pub loan_account_number: String,
    pub is_partially_paid: i32,
    pub reference_id: String,
    pub transaction_count: i32,
    pub repay_date: String,
    pub repay_amount: f32,
    pub repay_interest: f32,
    pub repay_interest_collected: f32,
    pub repay_os: f32,
    pub bank_settlement_date: String,
    pub repaid_at: String,
    pub interest_rate: f32,
    pub repay_principal: f32,
    pub repay_principal_collected: f32,
    pub repay_status: String,
    pub no_of_days: i32,
    pub repayment_type: String,
    pub revised_tenure: i32,
    pub revised_emi: f32,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Applicant {
    pub finternet_id: i32,
    pub applicant_type: String,
    pub applicant_name: String,
    pub applicant_number: String,
    pub pan: String,
    pub customer_category: String,
    pub gender: String,
    pub dob: String,
    pub marital_status: String,
    pub relation_with_applicant: String,
    pub father_name: String,
    pub mother_name: String,
    pub spouse_name: String,
    pub state: String,
    pub address_line_1: String,
    pub email_id: String,
    pub contact_no: String,
    pub company_name: String,
    pub company_category: String,
    pub salary: f32,
    pub constitution: String,
    pub gstin: String,
    pub credit_score: i32,
    pub foir: f32,
    pub incorp_date: String,
    pub industry_classify: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Transaction {
    pub transaction_reference: String,
    pub repaid_at: String,
    pub transaction_amount: f32,
    pub repayment_reference: String,
    pub settled_at: String,
    pub amount_adjusted_in_principal: f32,
    pub amount_adjusted_in_interest: f32,
    pub amount_adjusted_in_foreclosure_charges: f32,
    pub amount_adjusted_in_overdue_charges: f32,
    pub amount_adjusted_in_bounce_charges: f32,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Charge {
    pub reference_id: String,
    pub charge_id: String,
    pub charges_type: String,
    pub collected_at: String,
    pub settled_at: String,
    pub charges_amount: f32,
    pub collection_status: String,
    pub transaction_reference: String,
    pub tax_rate: i32,
    pub cgst: f32,
    pub invoice_value: f32,
    pub igst: f32,
    pub sgst: f32,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct InvestorRps {
    pub finternet_pool_id: i32,
    pub finternet_loan_id: i32,
    pub finternet_trustee_id: String,
    pub finternet_investor_id: i32,
    pub repayment_id: i32,
    pub pool_interest: f32,
    pub idr: f32,
    pub pdr: f32,
    pub repay_amount: f32,
    pub repay_date: String,
    pub repay_principal: f32,
    pub repay_principal_collected: f32,
    pub repayment_type: String,
    pub eis: f32,
    pub transaction_utr: String,
    pub repay_os: f32,
    pub repaid_at: String,
    pub repay_interest: f32,
    pub repay_interest_collected: f32,
    pub repay_status: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct OriginatorRps {
    pub finternet_pool_id: i32,
    pub finternet_loan_id: i32,
    pub finternet_trustee_id: String,
    pub idr: f32,
    pub finternet_originator_id: i32,
    pub repayment_id: i32,
    pub pool_interest: f32,
    pub pdr: f32,
    pub repay_date: String,
    pub repay_amount: f32,
    pub repay_principal: f32,
    pub repay_principal_collected: f32,
    pub repayment_type: String,
    pub eis: f32,
    pub repay_status: String,
    pub repay_os: f32,
    pub repay_interest: f32,
    pub repay_interest_collected: f32,
    pub eis_collected: f32,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct InvestorCharge {
    pub finternet_loan_id: String,
    pub finternet_repayment_id: String,
    pub finternet_pool_id: String,
    pub finternet_investor_id: String,
    pub finternet_charge_id: String,
    pub charges_reference_id: String,
    pub charges_type: String,
    pub collection_status: String,
    pub collected_at: String,
    pub cgst: f32,
    pub invoice_value: f32,
    pub document_type: String,
    pub charges_amount: f32,
    pub charges_utr: String,
    pub igst: f32,
    pub sgst: f32,
    pub document_number: String,
    pub document_date: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct LenderCharge {
    pub finternet_loan_id: String,
    pub finternet_repayment_id: String,
    pub finternet_pool_id: String,
    pub finternet_lender_id: String,
    pub finternet_charge_id: String,
    pub charges_reference_id: String,
    pub charges_type: String,
    pub collection_status: String,
    pub cgst: f32,
    pub invoice_value: f32,
    pub document_type: String,
    pub charges_amount: i32,
    pub igst: f32,
    pub sgst: f32,
    pub document_number: String,
    pub document_date: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PaymentDue {
    pub finternet_pool_id: String,
    pub finternet_loan_id: String,
    pub finternet_originator_id: String,
    pub finternet_investor_id: String,
    pub payout_id: String,
    pub due_amount: f32,
    pub due_amount_including_gst: f32,
    pub igst: f32,
    pub sgst: f32,
    pub cgst: f32,
    pub repay_interest: f32,
    pub repay_principal: f32,
    pub repay_os: f32,
    pub due_date: String,
    pub collected_at: String,
    pub transaction_id: String,
    pub collection_type: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PoolPayout {
    pub id: String,
    pub finternet_pool_id: String,
    pub gross_payout_amount: f32,
    pub net_payout_amount: f32,
    pub tds_amount: i32,
    pub transaction_utr: String,
    pub repaid_at: String,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Trustee {
    pub company_name: String,
    pub mobile_number: String,
    pub email_id: String,
    pub address_line1: String,
    pub address_line2: String,
    pub city: String,
    pub state: String,
    pub pincode: String,
    pub country: String,
}
