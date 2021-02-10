/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<i32>, _function: F) -> Vec<i32>
where
    F: FnMut(T) -> U,
{
    unimplemented!("Transform input vector {:?} using passed function", input);
}
