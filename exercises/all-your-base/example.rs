pub type Digit = u32;

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert<P: AsRef<[Digit]>>(digits: P, from_base: Digit, to_base: Digit) -> Result<Vec<Digit>, &'static str> {
    // check that both bases are in the correct range
    if from_base < 2 || to_base < 2 {
        return Err("Invalid base");
    }

    // check that all digits are in the correct range specified by the base
    if digits.as_ref().iter().any(|&num| num >= from_base) {
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
