pub fn rotate(text: &str, shift_key: u8) -> String {
    text.chars()
        .map(|c| {
            let case = if c.is_uppercase() { 'A' } else { 'a' } as u8;
            if c.is_alphabetic() {
                (((c as u8 - case + shift_key) % 26) + case) as char
            } else {
                c
            }
        })
        .collect::<String>()
}
