pub fn fix_billing_summary<T: std::ops::Add<Output = T>>(mut summary: Vec<T>) -> Vec<T> {
    assert!(summary.len() >= 2);

    let last_monthly_bill = summary.pop().unwrap();
    let final_bill = summary.pop().unwrap;
    let total_bill = last_monthly_bill + final_bill;
    summary.push(total_bill);

    summary
}
