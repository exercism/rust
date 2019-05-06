pub fn number(user_number: &str) -> Option<String> {
    let mut filtered_number: String = user_number.chars().filter(|ch| ch.is_digit(10)).collect();

    let number_len = filtered_number.len();

    if number_len < 10
        || number_len > 11
        || (filtered_number.len() == 11 && filtered_number.chars().nth(0).unwrap() != '1')
    {
        return None;
    }

    if number_len == 11 {
        filtered_number = filtered_number.chars().skip(1).collect();
    }

    if filtered_number
        .chars()
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap()
        < 2
        || filtered_number
            .chars()
            .nth(3)
            .unwrap()
            .to_digit(10)
            .unwrap()
            < 2
    {
        return None;
    }

    Some(filtered_number)
}
