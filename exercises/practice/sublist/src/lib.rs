#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    todo!(
        "Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}."
    );
}
