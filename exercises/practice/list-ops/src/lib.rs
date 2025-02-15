use std::fmt::Debug;

pub fn append(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    todo!("Append the elements in {b:?} to {a:?}");
}

pub fn concat<T: Debug>(list: Vec<Vec<T>>) -> Vec<T> {
    todo!("Create a list of the items inside each list in {list:?}")
}

pub fn filter(list: Vec<u32>, _function: impl Fn(u32) -> bool) -> Vec<u32> {
    todo!("Remove items that don't pass the filter function from {list:#?}")
}

pub fn length(list: Vec<u32>) -> usize {
    todo!("Calculate the length of {list:?}")
}

pub fn map(list: Vec<u32>, _function: impl Fn(u32) -> u32) -> Vec<u32> {
    todo!("Return the result of running the map function over items in {list:?}")
}

pub fn foldl(list: Vec<f64>, initial: f64, _function: impl Fn(f64, f64) -> f64) -> f64 {
    todo!("Build a value starting from {initial} and applying the function on each item in {list:?} left-to-right")
}

pub fn foldr(list: Vec<f64>, initial: f64, _function: impl Fn(f64, f64) -> f64) -> f64 {
    todo!("Build a value starting from {initial} and applying the function on each item in {list:?} right-to-left")
}

pub fn reverse<T: Debug>(mut list: Vec<T>) -> Vec<T> {
    todo!("Reverse {list:?}")
}
