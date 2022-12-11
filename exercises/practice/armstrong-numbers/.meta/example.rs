pub fn is_armstrong_number(num: u32) -> bool {
    let s = format!("{}", num);
    let l = s.len();
    s.chars()
        .map(|c| c.to_digit(10).unwrap().pow(l as u32))
        .try_fold(0u32, u32::checked_add)
        == Some(num)
}
