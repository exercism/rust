/// An ISBN type
#[derive(PartialEq, Eq)]
enum IsbnType {
    Isbn10,
    Isbn13,
}

/// Checks if an 'X' is valid at the given position for the given ISBN type
#[allow(non_snake_case)]
fn is_X_valid(position: &usize, isbn_type: &IsbnType) -> bool {
    (isbn_type == &IsbnType::Isbn10 && position == &9)
        || (isbn_type == &IsbnType::Isbn13 && position == &12)
}

/// Checks if a '-' is valid at the given position for the given ISBN type
fn is_dash_valid(position: &usize, isbn_type: &IsbnType) -> bool {
    isbn_type == &IsbnType::Isbn13 && (position == &1 || position == &5 || position == &11)
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_type = match isbn.len() {
        10 => IsbnType::Isbn10,
        13 => IsbnType::Isbn13,
        _ => return false,
    };

    let mut checksum = 0;
    let mut coefficient = 10;
    for (position, c) in isbn.char_indices() {
        let digit_value = match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'X' if is_X_valid(&position, &isbn_type) => 10,
            '-' if is_dash_valid(&position, &isbn_type) => continue,
            _ => return false,
        };

        checksum += coefficient * digit_value;
        coefficient -= 1;
    }

    checksum % 11 == 0
}
