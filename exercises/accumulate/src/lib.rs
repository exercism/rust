pub fn map_function<T, R>(_xs: Vec<T>, _f: &Fn(T) -> R) -> Vec<R> {
    unimplemented!()
}

pub fn map_closure<F, T, R>(_xs: Vec<T>, _f: F) -> Vec<R>
where
    F: Fn(T) -> R,
{
    unimplemented!()
}
