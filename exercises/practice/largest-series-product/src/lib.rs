#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    // `windows(0) not allowed`, so handling it here
    if span == 0 {
        return Ok(1);
    }

    let mut max_product: u64 = 0;

    for window in string_digits
    .chars()
    .collect::<Vec<char>>()
    .windows(span) {
        let mut product: u64 = 1;
        
        for c in window {
            if let Some(d) = c.to_digit(10) {
                product *= d as u64;
            }
            else {
                return Err(Error::InvalidDigit(*c));
            }
        }

        if product > max_product {
            max_product = product;
        }
    }

    Ok(max_product)   
}
