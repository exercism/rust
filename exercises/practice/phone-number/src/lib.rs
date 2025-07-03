pub fn number(user_number: &str) -> Option<String> {
    let mut digits: Vec<u8> = user_number
    .chars()
    .filter(|c| c.is_numeric())
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect();

    // Handle potential country code
    if digits.get(0) == Some(&1) {
        digits.remove(0); 
    }

    // If not 10 digits after removing potential country code, then invalid phone number
    if digits.len() != 10 {
        return None;
    }

    // Check N for area code
    match digits.get(0) {
        Some(2..=9) => (),
        _ => {
            return None;
        }
    }

    // Check N for local code   
    match digits.get(3) {
        Some(2..=9) => (),
        _ => {
            return None;
        }
    }        

    Some(digits.into_iter().map(|c| c.to_string()).collect())
}
