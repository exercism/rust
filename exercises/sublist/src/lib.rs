use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    unimplemented!("Determine if the first list '{:?}' is equal to, sublist of, superlist of or unequal to the second list '{:?}'", first_list, second_list);
}
