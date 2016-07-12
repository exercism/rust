pub type Digit = i64;

pub fn convert<P: AsRef<[Digit]>>(digits: P, from_base: Digit, to_base: Digit) -> Result<Vec<Digit>, &'static str> {
    // check that both bases are in the correct range
    if from_base < 2 || to_base < 2 {
        return Err("Invalid base");
    }

    // check that all digits are in the correct range specified by the base
    if digits.as_ref().iter().any(|&num| num < 0 || num >= from_base) {
        return Err("Digits invalid for input base");
    }

    // convert all digits into a single large number
    let mut immediate = digits.as_ref().iter()
        .rev()
        .enumerate()
        .map(|(i, &num)| num * from_base.pow(i as u32))
        .fold(0, |accu, num| accu + num);

    // convert number into digits
    let mut res = Vec::new();
    while immediate > 0 {
        res.push(immediate % to_base);
        immediate /= to_base;
    }
    // fix order of digits
    res.reverse();
    Ok(res)
}
