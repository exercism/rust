fn ascii(ch: char) -> u8 {
    ch as u8
}

fn get_transpose(ch: char) -> char {
    if ch.is_digit(10) {
        ch
    } else {
        (ascii('z') - ascii(ch) + ascii('a')) as char
    }
}

pub fn encode(plaintext: &str) -> String {
    plaintext
        .to_lowercase()
        .chars()
        .filter(|&ch| ch.is_ascii())
        .filter(|&ch| ch.is_alphanumeric())
        .map(get_transpose)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|slice| slice.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext
        .split::<char>(' ')
        .collect::<String>()
        .chars()
        .map(get_transpose)
        .collect::<String>()
}
