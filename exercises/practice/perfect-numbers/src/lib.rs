use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }

    let mut aliquot_sum = 0;
    for i in 1..num {
        if num % i == 0 {
            aliquot_sum += i;
        }
    }

    match aliquot_sum.cmp(&num) {
        Ordering::Less => {
            Some(Classification::Deficient)
        },
        Ordering::Greater => {
            Some(Classification::Abundant)
        },
        Ordering::Equal => {
            Some(Classification::Perfect)
        }
    }
}
