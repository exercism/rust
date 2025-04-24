/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars().filter(|c| !c.is_whitespace()).rev().try_fold((0, 0), |(i, checksum), c| {
        c.to_digit(10).map(|d| {
            println!("Doubling every two digits : {} {} {}", i, checksum, d);
            if i%2 == 1 {
                d *2
            }
            else {
                d
            }
        })
        .map(|d| {
            println!("Substract 9 if greater than 9 : {} {} {}", i, checksum, d);
            if d > 9 {
                d - 9
            }
            else {
                d
            }
        })
        .map(|d| {
            println!("In the accumulator : {} {} {}", i, checksum, d);
            (i+1, checksum + d)
        })
    })
    .is_some_and(|(len, checksum)| len > 1 && checksum % 10 == 0)
}
