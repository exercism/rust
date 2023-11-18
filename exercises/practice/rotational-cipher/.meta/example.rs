pub fn rotate(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let case = if c.is_uppercase() { 'A' } else { 'a' } as u8;

                (((c as u8 - case + key) % 26) + case) as char
            } else {
                c
            }
        })
        .collect()
}
