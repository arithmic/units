//! Loan Pool Token Workflow Example
//!
//! This example demonstrates how loan pool tokens work within the workflow layer,
//! showing pool creation, investor management, and payout processing.

use std::time::{SystemTime, UNIX_EPOCH};

use tokens::{
    loantypes::{PaymentDue, PoolPayout, Trustee},
    loan_pool_token::{LoanPoolToken, MintLoanPoolTokenInput, UpdateLoanPoolTokenInput},
};
use workflow_layer::{
    save_transaction_log_to_database,
    commit_global_state,
    GlobalStateSMT,
};

fn main() {
    println!("🏦 Loan Pool Token Workflow Demo");
    println!("=================================");

    // Create an instance of the global state SMT
    let mut global_state_smt = GlobalStateSMT::default();
    println!("🚀 Initial SMT root: {:?}", global_state_smt.root());

    if let Err(e) = execute_pool_workflow(&mut global_state_smt) {
        println!("❌ Failed: {}", e);
        return;
    }

    println!("✅ Loan Pool Token Workflow Demo Complete");
}

fn execute_pool_workflow(smt: &mut GlobalStateSMT) -> Result<(), String> {
    // Step 1: Create loan pool token
    println!("1. Creating loan pool token...");
    
    let sample_trustee = Trustee {
        company_name: "secure trustee services".to_string(),
        mobile_number: "+1-555-0199".to_string(),
        email_id: "trustee@secureservices.com".to_string(),
        address_line1: "789 trustee avenue".to_string(),
        address_line2: "suite 200".to_string(),
        city: "new york".to_string(),
        state: "new york".to_string(),
        pincode: "10001".to_string(),
        country: "usa".to_string(),
    };

    let sample_payment_due = PaymentDue {
        finternet_pool_id: "POOL001".to_string(),
        finternet_loan_id: "LOAN001".to_string(),
        finternet_originator_id: "ORG001".to_string(),
        finternet_investor_id: "INV001".to_string(),
        payout_id: "PAY001".to_string(),
        due_amount: 10000.0,
        due_amount_including_gst: 11800.0,
        igst: 1800.0,
        sgst: 0.0,
        cgst: 0.0,
        repay_interest: 2000.0,
        repay_principal: 8000.0,
        repay_os: 92000.0,
        due_date: "2024-03-15".to_string(),
        collected_at: "2024-03-15T10:00:00Z".to_string(),
        transaction_id: "TXN001".to_string(),
        collection_type: "automatic".to_string(),
        status: "pending".to_string(),
    };

    let sample_pool_payout = PoolPayout {
        id: "PAYOUT001".to_string(),
        finternet_pool_id: "POOL001".to_string(),
        gross_payout_amount: 10000.0,
        net_payout_amount: 9000.0,
        tds_amount: 1000,
        transaction_utr: "UTR123456789".to_string(),
        repaid_at: "2024-03-15T15:30:00Z".to_string(),
    };

    let mint_input = MintLoanPoolTokenInput {
        pool_name: "diversified income pool q1 2024".to_string(),
        pool_cut_off_date: "2024-01-31".to_string(),
        status: "active".to_string(),
        broken_days: 0,
        pool_count: 25,
        pool_amount: 2500000.0,
        eis_method: "daily".to_string(),
        finternet_originator_id: "ORG001".to_string(),
        lender_share: 0.15,
        lender_os: 2500000.0,
        finternet_trustee_id: "TRUSTEE001".to_string(),
        finternet_investor_id: "INV001".to_string(),
        investor_share: 0.85,
        investor_os: 2125000.0,
        payout_days: 30,
        fees_type: "percentage".to_string(),
        total_fees: 25000.0,
        pool_xirr: 12.5,
        investor_fees: 20000.0,
        direct_pricing: 11.8,
        pool_cycle: "monthly".to_string(),
        pool_health: "healthy".to_string(),
        interest_method: "reducing".to_string(),
        purchase_date: "2024-01-01".to_string(),
        investor_repayment_type: "bullet".to_string(),
        loan: vec![
            "LOAN001".to_string(),
            "LOAN002".to_string(),
            "LOAN003".to_string(),
        ],
        payment_due: vec![sample_payment_due.clone()],
        pool_payout: vec![sample_pool_payout.clone()],
        trustee: sample_trustee,
    };

    let mut pool_token = LoanPoolToken::mint(mint_input);
    println!("   ✅ Created pool: {}", pool_token.pool_name);
    println!("   💰 Pool amount: ${}", pool_token.pool_amount);
    println!("   📊 Status: {}", pool_token.status);
    println!("   🏦 Loan count: {}", pool_token.loan.len());

    // Step 2: Process monthly collections
    println!("2. Processing monthly collections...");
    
    // Simulate collection of payments
    let collections_amount = 150000.0;
    let new_pool_amount = pool_token.pool_amount + collections_amount;
    
    let update_input = UpdateLoanPoolTokenInput {
        pool_name: pool_token.pool_name.clone(),
        pool_amount: Some(new_pool_amount),
        status: Some("performing".to_string()),
        pool_count: Some(pool_token.pool_count + 5), // New loans added
        pool_cut_off_date: None,
        broken_days: None,
        eis_method: None,
        finternet_originator_id: None,
        lender_share: None,
        lender_os: None,
        finternet_trustee_id: None,
        finternet_investor_id: None,
        investor_share: None,
        investor_os: None,
        payout_days: None,
        fees_type: None,
        total_fees: None,
        pool_xirr: None,
        investor_fees: None,
        direct_pricing: None,
        pool_cycle: None,
        pool_health: None,
        interest_method: None,
        purchase_date: None,
        investor_repayment_type: None,
        loan: None,
        payment_due: None,
        pool_payout: None,
        trustee: None,
    };

    pool_token.update(update_input);
    println!("   ✅ Collections processed");
    println!("   💰 New pool amount: ${}", pool_token.pool_amount);
    println!("   📊 Status: {}", pool_token.status);
    println!("   🏦 Loan count: {}", pool_token.pool_count);

    // Step 3: Calculate investor payouts
    println!("3. Calculating investor payouts...");
    
    let monthly_return = pool_token.pool_amount * (pool_token.pool_xirr / 100.0 / 12.0);
    let investor_payout = monthly_return * pool_token.investor_share;
    let lender_payout = monthly_return * pool_token.lender_share;
    
    println!("   ✅ Payouts calculated");
    println!("   💰 Total monthly return: ${:.2}", monthly_return);
    println!("   👥 Investor payout: ${:.2}", investor_payout);
    println!("   🏦 Lender payout: ${:.2}", lender_payout);

    // Step 4: Process trustee operations
    println!("4. Processing trustee operations...");
    
    println!("   📋 Trustee: {}", pool_token.trustee.company_name);
    println!("   📍 Location: {}, {}", pool_token.trustee.city, pool_token.trustee.state);
    println!("   📧 Contact: {}", pool_token.trustee.email_id);
    println!("   ✅ Trustee operations verified");

    // Step 5: Save to database (simulated)
    println!("5. Saving pool data to database...");
    // Note: In a real implementation, this would serialize the pool token
    // and save it using the workflow layer database functions
    println!("   ✅ Pool data saved");

    // Step 6: Commit to global state (simulated)
    println!("6. Committing to global state...");
    // Note: In a real implementation, this would update the SMT with pool state
    println!("   ✅ State committed");

    // Step 7: Generate pool reports
    println!("7. Generating pool reports...");
    print_pool_summary(&pool_token);
    print_pool_performance(&pool_token);
    println!("   ✅ Reports generated");

    Ok(())
}

fn print_pool_summary(pool: &LoanPoolToken) {
    println!("   📋 Pool Summary:");
    println!("      Name: {}", pool.pool_name);
    println!("      Pool amount: ${}", pool.pool_amount);
    println!("      Loan count: {}", pool.pool_count);
    println!("      Status: {}", pool.status);
    println!("      XIRR: {}%", pool.pool_xirr);
    println!("      Investor share: {}%", (pool.investor_share * 100.0));
    println!("      Lender share: {}%", (pool.lender_share * 100.0));
    println!("      Trustee: {}", pool.trustee.company_name);
}

fn print_pool_performance(pool: &LoanPoolToken) {
    println!("   📊 Performance Metrics:");
    println!("      Pool health: {}", pool.pool_health);
    println!("      Broken days: {}", pool.broken_days);
    println!("      Interest method: {}", pool.interest_method);
    println!("      Repayment type: {}", pool.investor_repayment_type);
    println!("      Payout cycle: {} days", pool.payout_days);
    println!("      Total fees: ${}", pool.total_fees);
    
    if !pool.payment_due.is_empty() {
        println!("      Pending payments: {}", pool.payment_due.len());
        let total_due: f32 = pool.payment_due.iter()
            .map(|p| p.due_amount)
            .sum();
        println!("      Total amount due: ${}", total_due);
    }
    
    if !pool.pool_payout.is_empty() {
        println!("      Completed payouts: {}", pool.pool_payout.len());
        let total_payout: f32 = pool.pool_payout.iter()
            .map(|p| p.net_payout_amount)
            .sum();
        println!("      Total payouts: ${}", total_payout);
    }
}