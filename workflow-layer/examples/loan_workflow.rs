//! Loan Token Workflow Example
//!
//! This example demonstrates how loan tokens work within the workflow layer,
//! showing loan creation, updates, and basic operations.

use std::time::{SystemTime, UNIX_EPOCH};

use tokens::{
    loantypes::{Applicant, Repayment},
    loan_token::{LoanToken, MintLoanTokenInput, UpdateLoanTokenInput},
};
use workflow_layer::{
    save_transaction_log_to_database,
    commit_global_state,
    GlobalStateSMT,
};
use sparse_merkle_tree::H256;

fn main() {
    println!("🏦 Loan Token Workflow Demo");
    println!("===========================");

    // Create an instance of the global state SMT
    let mut global_state_smt = GlobalStateSMT::default();
    println!("🚀 Initial SMT root: {:?}", global_state_smt.root());

    if let Err(e) = execute_loan_workflow(&mut global_state_smt) {
        println!("❌ Failed: {}", e);
        return;
    }

    println!("🔄 Final SMT root: {:?}", global_state_smt.root());
    println!("✅ Loan Token Workflow Demo Complete");
}

fn execute_loan_workflow(smt: &mut GlobalStateSMT) -> Result<(), String> {
    // Step 1: Create loan token data
    println!("1. Creating loan token...");
    
    let sample_applicant = Applicant {
        finternet_id: 12345,
        applicant_type: "primary".to_string(),
        applicant_name: "john doe".to_string(),
        applicant_number: "APP001".to_string(),
        pan: "ABCDE1234F".to_string(),
        customer_category: "individual".to_string(),
        gender: "male".to_string(),
        dob: "1990-01-15".to_string(),
        marital_status: "single".to_string(),
        relation_with_applicant: "self".to_string(),
        father_name: "robert doe".to_string(),
        mother_name: "jane doe".to_string(),
        spouse_name: "".to_string(),
        state: "california".to_string(),
        address_line_1: "123 main street".to_string(),
        email_id: "john.doe@example.com".to_string(),
        contact_no: "+1-555-0123".to_string(),
        company_name: "tech corp".to_string(),
        company_category: "software".to_string(),
        salary: 75000.0,
        constitution: "individual".to_string(),
        gstin: "".to_string(),
        credit_score: 750,
        foir: 0.35,
        incorp_date: "".to_string(),
        industry_classify: "technology".to_string(),
    };

    let sample_repayment = Repayment {
        loan_account_number: "LOAN001".to_string(),
        is_partially_paid: 0,
        reference_id: "REP001".to_string(),
        transaction_count: 1,
        repay_date: "2024-02-15".to_string(),
        repay_amount: 5000.0,
        repay_interest: 1000.0,
        repay_interest_collected: 1000.0,
        repay_os: 95000.0,
        bank_settlement_date: "2024-02-15".to_string(),
        repaid_at: "2024-02-15T10:30:00Z".to_string(),
        interest_rate: 8.5,
        repay_principal: 4000.0,
        repay_principal_collected: 4000.0,
        repay_status: "paid".to_string(),
        no_of_days: 30,
        repayment_type: "emi".to_string(),
        revised_tenure: 60,
        revised_emi: 5000.0,
    };

    let mint_input = MintLoanTokenInput {
        loan_account_number: "LOAN001".to_string(),
        sanction_amount: 100000.0,
        loan_amount: 100000.0,
        disbursement_amount: 100000.0,
        interest_rate: 8.5,
        tenure: 60,
        investor_os: 100000.0,
        cersai_date: "2024-01-15".to_string(),
        loan_rescheduled: 0,
        cersai_registration_number: "CERSAI001".to_string(),
        sourcing_branch: "main branch".to_string(),
        loan_type: "personal".to_string(),
        program: "standard".to_string(),
        status: "disbursed".to_string(),
        product_type: "personal loan".to_string(),
        transaction_type: "loan".to_string(),
        product_code: "PL001".to_string(),
        total_income: 90000.0,
        total_obligation: 15000.0,
        max_dpd: 0,
        loan_restructured: 0,
        sourcing_state: "california".to_string(),
        emi_day: 15,
        emi_per_month: 5000.0,
        disbursement_date: "2024-01-15".to_string(),
        roi_type: "fixed".to_string(),
        last_12_months_bounce_count: "0".to_string(),
        emi_paid: 0.0,
        balance: 100000.0,
        emi_bounced: "0".to_string(),
        property_type: "".to_string(),
        property_status: "".to_string(),
        collateral_location: "".to_string(),
        ltv: "".to_string(),
        property_value: 0.0,
        foir: 0.35,
        credit_score: 750,
        outstanding_balance: 100000.0,
        repayment_schedule: vec![sample_repayment.clone()],
        applicant: vec![sample_applicant],
        transaction: vec![],
        charges: vec![],
        investor_rps: vec![],
        originator_rps: vec![],
        investor_charges: vec![],
        lender_charges: vec![],
    };

    let mut loan_token = LoanToken::mint(mint_input);
    println!("   ✅ Created loan: {}", loan_token.loan_account_number);
    println!("   💰 Amount: ${}", loan_token.loan_amount);
    println!("   📊 Status: {}", loan_token.status);

    // Step 2: Process EMI payment
    println!("2. Processing EMI payment...");
    
    let update_input = UpdateLoanTokenInput {
        loan_account_number: "LOAN001".to_string(),
        status: Some("active".to_string()),
        emi_paid: Some(5000.0),
        balance: Some(95000.0),
        outstanding_balance: Some(95000.0),
        sanction_amount: None,
        loan_amount: None,
        disbursement_amount: None,
        interest_rate: None,
        tenure: None,
        investor_os: None,
        cersai_date: None,
        loan_rescheduled: None,
        cersai_registration_number: None,
        sourcing_branch: None,
        loan_type: None,
        program: None,
        product_type: None,
        transaction_type: None,
        product_code: None,
        total_income: None,
        total_obligation: None,
        max_dpd: None,
        loan_restructured: None,
        sourcing_state: None,
        emi_day: None,
        emi_per_month: None,
        disbursement_date: None,
        roi_type: None,
        last_12_months_bounce_count: None,
        emi_bounced: None,
        property_type: None,
        property_status: None,
        collateral_location: None,
        ltv: None,
        property_value: None,
        foir: None,
        credit_score: None,
        repayment_schedule: None,
        applicant: None,
        transaction: None,
        charges: None,
        investor_rps: None,
        originator_rps: None,
        investor_charges: None,
        lender_charges: None,
    };

    loan_token.update(update_input);
    println!("   ✅ Payment processed");
    println!("   💰 EMI paid: ${}", loan_token.emi_paid);
    println!("   📊 Remaining balance: ${}", loan_token.balance);

    // Step 3: Save to database (simulated)
    println!("3. Saving loan data to database...");
    // Note: In a real implementation, this would serialize the loan token
    // and save it using the workflow layer database functions
    println!("   ✅ Loan data saved");

    // Step 4: Commit to global state
    println!("4. Committing to global state...");
    
    // Create a hash key for the loan
    let loan_key_str = format!("loan:{}", loan_token.loan_account_number);
    let mut key_bytes = [0u8; 32];
    for (i, byte) in loan_key_str.bytes().enumerate() {
        if i < 32 {
            key_bytes[i] = byte;
        }
    }
    let loan_key = H256::from(key_bytes);
    
    // Create a hash value for the loan data
    let loan_value_str = format!("{}:{}:{}", loan_token.outstanding_balance, loan_token.status, loan_token.emi_paid);
    let mut value_bytes = [0u8; 32];
    for (i, byte) in loan_value_str.bytes().enumerate() {
        if i < 32 {
            value_bytes[i] = byte;
        } else {
            value_bytes[i % 32] ^= byte; // Mix additional bytes
        }
    }
    let loan_value = H256::from(value_bytes);
    
    smt.update(loan_key, loan_value)
        .map_err(|e| format!("failed to update smt: {:?}", e))?;
    
    println!("   ✅ State committed");

    // Step 5: Generate reports
    println!("5. Generating loan reports...");
    print_loan_summary(&loan_token);
    println!("   ✅ Reports generated");

    Ok(())
}

fn print_loan_summary(loan: &LoanToken) {
    println!("   📋 Loan Summary:");
    println!("      Account: {}", loan.loan_account_number);
    println!("      Borrower: {}", loan.applicant.first().map_or("N/A".to_string(), |a| a.applicant_name.clone()));
    println!("      Original amount: ${}", loan.loan_amount);
    println!("      Outstanding balance: ${}", loan.outstanding_balance);
    println!("      Interest rate: {}%", loan.interest_rate);
    println!("      EMI: ${}", loan.emi_per_month);
    println!("      Status: {}", loan.status);
    println!("      Total payments: {}", loan.repayment_schedule.len());
}