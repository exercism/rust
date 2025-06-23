/// What should the type of _function be?
pub fn map<T, U, F>(input: Vec<T>, mut _function: F ) -> Vec<U> 
where
    F: FnMut(T) -> U,
{
    let mut res = Vec::new();

    for el in input {
        res.push(_function(el));
    }
    res
}
