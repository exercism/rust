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
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .windows(span)
        .map(|w| w.into_iter().product())
        .collect();

    if let Some(&x) = products.iter().max() {
        Ok(x)
    } else {
        Err(Error::SpanTooLong)
    }
}
