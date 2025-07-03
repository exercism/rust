use once_cell::sync::Lazy;

static ALPHABET: Lazy<Vec<char>> = Lazy::new(|| {
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 
    'y', 'z'].into_iter().collect()
});

pub fn rotate(input: &str, key: u8) -> String {
    let mut rotated_alphabet: Vec<char> = ALPHABET.clone();
    rotated_alphabet.rotate_left(key as usize);

    input
    .chars()
    .map(|c| {
        if c.is_alphabetic() {
            rotated_alphabet
            .get(ALPHABET.iter().position(|&cc| cc == c.to_ascii_lowercase()).unwrap())
            .map(|rot_c| {
                if c.is_uppercase() {
                    rot_c.to_ascii_uppercase()
                }
                else {
                    *rot_c
                }
            })
            .unwrap()
        }
        else {
            c
        }
    })
    .collect()
}
