pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        if self.chars().any(|c| c.is_alphabetic()) || self.chars().count() == 1 {
            return false;
        }

        self.chars()
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

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        String::from(*self).valid_luhn()
    }
}

impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}
