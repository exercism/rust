pub fn fix_billing_summary(mut summary: Vec<i32>) -> Vec<i32> {
    assert!(summary.len() >= 2);
    
    let last_monthly_bill = summary.pop().unwrap();
    let final_bill = summary.pop().unwrap;
    let total_bill = last_monthly_bill + final_bill;
    summary.push(total_bill);

    summary;
}
