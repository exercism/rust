#[derive(PartialEq, Eq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if contains(a, b) {
        Comparison::Superlist
    } else if contains(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() < b.len() {
        return false;
    }

    if a.starts_with(b) {
        return true;
    }

    contains(&a[1..], b)
}
