pub fn map_function<T, R>(_xs: Vec<T>, _f: &Fn(T) -> R) -> Vec<R> {
    panic!("Implement your solution here")
}

pub fn map_closure<F, T, R>(_xs: Vec<T>, _f: F) -> Vec<R>
where
    F: Fn(T) -> R,
{
    panic!("Implement your solution here")
}
