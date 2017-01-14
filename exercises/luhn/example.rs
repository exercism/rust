pub fn is_valid(candidate: &str) -> bool {
    if candidate.chars().any(|c| c.is_alphabetic()) || candidate.chars().count() == 1 {
        return false;
    }

    candidate.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .rev()
        .enumerate()
        .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
        .map(|digit| if digit > 10 { digit - 9 } else { digit })
        .sum::<u32>() % 10 == 0
}
