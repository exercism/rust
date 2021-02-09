#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if let Some(invalid) = string_digits.chars().find(|c| !c.is_digit(10)) {
        return Err(Error::InvalidDigit(invalid));
    }

    let products: Vec<u64> = string_digits
        .chars()
        .map(|c| u64::from(c.to_digit(10).unwrap()))
        .collect::<Vec<u64>>()
        .windows(span)
        .map(|w| w.iter().product())
        .collect();

    if let Some(&x) = products.iter().max() {
        Ok(x)
    } else {
        Err(Error::SpanTooLong)
    }
}
