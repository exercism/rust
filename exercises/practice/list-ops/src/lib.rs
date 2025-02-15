use std::fmt::Debug;

pub fn append(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    todo!("add all items in {b:?} to the end of the {a:?}");
}

pub fn concat<T: Debug>(list: Vec<Vec<T>>) -> Vec<T> {
    todo!("combine all items in all lists inside {list:?} into one flattened list")
}

pub fn filter(list: Vec<i32>, _predicate: impl Fn(i32) -> bool) -> Vec<i32> {
    todo!("return the list of all items in {list:?} for which `predicate(item)` is true")
}

pub fn length(list: Vec<i32>) -> usize {
    todo!("return the total number of items within {list:?}")
}

pub fn map(list: Vec<i32>, _function: impl Fn(i32) -> i32) -> Vec<i32> {
    todo!("return the list of the results of applying `function(item)` on all {list:?} items")
}

pub fn foldl(list: Vec<f64>, initial: f64, _function: impl Fn(f64, f64) -> f64) -> f64 {
    todo!("fold (reduce) each {list:?} item into the accumulator from the left")
}

pub fn foldr(list: Vec<f64>, initial: f64, _function: impl Fn(f64, f64) -> f64) -> f64 {
    todo!("fold (reduce) each {list:?} item into the accumulator from the right")
}

pub fn reverse<T: Debug>(mut list: Vec<T>) -> Vec<T> {
    todo!("Return {list:?} with al the original items, but in reverse order")
}
