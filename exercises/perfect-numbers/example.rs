pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum: u64 = (1..num).filter(|i| num%i == 0).sum();
    if sum == num {
        Some(Classification::Perfect)
    } else if sum < num {
        Some(Classification::Deficient)
    } else {
        Some(Classification::Abundant)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}
