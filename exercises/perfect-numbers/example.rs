pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }
    let sum: u64 = (1..num).filter(|i| num%i == 0).sum();
    if sum == num {
        Ok(Classification::Perfect)
    } else if sum < num {
        Ok(Classification::Deficient)
    } else {
        Ok(Classification::Abundant)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}