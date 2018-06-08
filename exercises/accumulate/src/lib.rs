pub fn map_function<T, R>(xs: Vec<T>, f: &Fn(T) -> R) -> Vec<R> {
    panic!("Implement your solution here")
}

pub fn map_closure<F, T, R>(xs: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> R,
{
    panic!("Implement your solution here")
}
