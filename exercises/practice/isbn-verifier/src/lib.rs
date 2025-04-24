/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    if isbn[..isbn.len() - 1].chars().any(|c| !( c.is_ascii_digit() || c=='-')) {
        return false;
    }

    if !isbn.chars().last().is_some_and(|c| c.is_ascii_digit() || c == 'X') {
        return false;
    }

    let temp: String = isbn.chars().filter(|&c| c!= '-').collect();

    println!("{}", temp);

    if temp.chars().count() != 10 {
        false
    }
    else {
        let qty:u32 = temp.chars().enumerate().map(|(i, c)| {
            if i<10-1 {
                c.to_digit(10).unwrap() * (10-i as u32)
            }
            else if c == 'X' {
                10
            }
            else {
                c.to_digit(10).unwrap()
            }
        }).sum();
        println!("{}", qty);
        qty % 11 == 0
    }
}
