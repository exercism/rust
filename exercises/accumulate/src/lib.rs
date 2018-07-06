pub fn map_function(input: Vec<i32>, _function: impl Fn(i32) -> i32) -> Vec<i32> {
    unimplemented!("Transform input vector {:?} using passed function", input);
}

pub fn map_closure(input: Vec<i32>, _closure: impl Fn(i32) -> i32) -> Vec<i32> {
    unimplemented!("Transform input vector {:?} using passed closure", input);
}
