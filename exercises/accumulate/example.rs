pub fn map_function(values: Vec<i32>, f: &Fn(i32) -> i32) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    for val in values {
        out.push(f(val))
    }
    out
}

pub fn map_closure<F>(values: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut out: Vec<i32> = vec![];
    for val in values {
        out.push(f(val))
    }
    out
}
