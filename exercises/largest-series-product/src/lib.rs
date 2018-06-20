#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    unimplemented!(
        "largest series product of a span of {} digits in {}",
        span,
        string_digits
    );
}
