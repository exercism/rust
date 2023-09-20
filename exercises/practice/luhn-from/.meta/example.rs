pub struct Luhn {
    digits: Vec<char>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.digits.iter().any(|c| c.is_alphabetic()) || self.digits.len() == 1 {
            return false;
        }

        self.digits
            .iter()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .map(|(index, digit)| if index % 2 == 0 { digit } else { digit * 2 })
            .map(|digit| if digit > 9 { digit - 9 } else { digit })
            .sum::<u32>()
            % 10
            == 0
    }
}

impl From<String> for Luhn {
    fn from(s: String) -> Self {
        Luhn {
            digits: s.chars().collect(),
        }
    }
}

impl<'a> From<&'a str> for Luhn {
    fn from(s: &'a str) -> Self {
        Luhn::from(String::from(s))
    }
}

impl From<u8> for Luhn {
    fn from(s: u8) -> Self {
        Luhn::from(s.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(s: u16) -> Self {
        Luhn::from(s.to_string())
    }
}

impl From<u32> for Luhn {
    fn from(s: u32) -> Self {
        Luhn::from(s.to_string())
    }
}

impl From<u64> for Luhn {
    fn from(s: u64) -> Self {
        Luhn::from(s.to_string())
    }
}

impl From<usize> for Luhn {
    fn from(s: usize) -> Self {
        Luhn::from(s.to_string())
    }
}
