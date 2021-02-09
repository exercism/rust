pub fn is_valid(candidate: &str) -> bool {
    if candidate.chars().filter(|c| c.is_digit(10)).take(2).count() <= 1
        || candidate.chars().any(|c| !c.is_digit(10) && c != ' ')
    {
        return false;
    }

    candidate
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
        .map(|digit| if digit > 9 { digit - 9 } else { digit })
        .sum::<u32>()
        % 10
        == 0
}
