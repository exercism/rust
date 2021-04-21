/// Combines the final two elements of the input vector, or `None` if fewer than two elements present.
pub fn fix_billing_summary<T: std::ops::Add<Output = T>>(mut summary: Vec<T>) -> Option<Vec<T>> {
    let final_bill = summary.pop()?;
    let last_monthly_bill = summary.pop()?;
    summary.push(last_monthly_bill + final_bill);
    Some(summary)
}
