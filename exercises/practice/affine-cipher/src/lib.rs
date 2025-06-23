use once_cell::sync::Lazy;

static NB_LETTERS_IN_ALPHABET: Lazy<i32> = Lazy::new(|| {
    26
});

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {

    match find_mmi(a, *NB_LETTERS_IN_ALPHABET) {
        Some(_) => {
            let res = plaintext
            .chars()
            .filter(|c| {
                c.is_alphanumeric()
            })
            .map(|c| {
                c.to_ascii_lowercase()
            })
            .map(|c| {
                if c.is_ascii_digit() {
                    c
                }
                else {
                    let i = c as u8 - b'a';
                    let letter_number: u8 = ((a * i as i32 + b ).rem_euclid(*NB_LETTERS_IN_ALPHABET)) as u8;
                    (b'a' + letter_number) as char
                }
            })
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|chunk| {
                chunk.iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ");

            Ok(res)
        },
        None => {
            Err(AffineCipherError::NotCoprime(a))
        }
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match find_mmi(a, *NB_LETTERS_IN_ALPHABET) {
        Some(mmi) => 
        {
            let res = ciphertext
            .chars()
            .filter(|c| {
                c.is_alphanumeric()
            })
            .map(|c| {
                if c.is_ascii_digit() {
                    c
                }
                else {
                    let y = c as u8 -b'a';
                    // needs `rem_euclid()` because % can give negative results
                    let d: u8 = ((mmi * (y as i32 - b)).rem_euclid(*NB_LETTERS_IN_ALPHABET)) as u8; 
                    (b'a' + d) as char
                }
            })
            .collect::<String>();

            Ok(res)
        },
        None => {
        // The MMI only exists if `a` and `NB_LETTERS_IN_ALPHABET` are coprime
            Err(AffineCipherError::NotCoprime(a))
        }
    }
}

fn find_mmi(a: i32, m: i32) -> Option<i32> {
    // cannot be greater than `m` (otherwise would be cancel out in the mod)
    (0..m).find(|x| (a * *x) % m == 1)
}