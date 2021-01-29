pub fn rotate(text: &str, key: i8) -> String {
    let key = if key < 0 { (26 + key) as u8 } else { key as u8 };

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
