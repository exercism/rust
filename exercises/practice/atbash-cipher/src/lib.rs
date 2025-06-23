use once_cell::sync::Lazy;

static ALPHABET: Lazy<&str> = Lazy::new(|| {
    "abcdefghijklmnopqrstuvwxyz"
});

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let nb_letters = ALPHABET.chars().count();

    plain
    .chars()
    .filter(|c| {
        c.is_alphanumeric()
    })
    .map(|c| {
        c.to_ascii_lowercase()
    })
    .map(|c| {
        if c.is_numeric() {
            c
        }
        else {
            ALPHABET.chars().nth(nb_letters - 1 - (c as u8 - b'a') as usize).unwrap()
        }
    })
    .collect::<Vec<char>>()
    .chunks(5)
    .map(|chunk| {
        chunk.iter().collect::<String>()
    })
    .collect::<Vec<String>>()
    .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encode(cipher)
    .chars()
    .filter(|c| {
        !c.is_whitespace()
    })
    .collect()
}
