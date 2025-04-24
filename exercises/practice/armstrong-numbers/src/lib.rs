pub fn is_armstrong_number(num: u32) -> bool {
    let nb_digit = num.to_string().chars().count();
    num.to_string().chars()
    .map(|c| c.to_digit(10).unwrap().pow(nb_digit as u32))
    .sum::<u32>() == num 
}
