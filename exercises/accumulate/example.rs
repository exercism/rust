pub fn map<F>(mut values: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    for val in &mut values {
        *val = f(*val);
    }
    values
}
