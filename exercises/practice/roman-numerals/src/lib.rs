use std::{fmt::{Display, Formatter, Result}};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let roman_map = [(1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I")];

        let mut res = String::new();
        let mut num = self.0;

        for &(divisor, roman) in roman_map.iter() {
            while num >= divisor {
                res.push_str(roman);
                num -= divisor;
            }
        }

        write!(_f, "{res}")

    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}
