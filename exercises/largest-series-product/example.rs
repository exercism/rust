use std::str::FromStr;

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, String> {
    if span == 0 {
        return Ok(1);
    }

    if string_digits.chars().any(|c| !c.is_numeric()) {
        return Err(String::from("All characters must be numbers"));
    }

    let products: Vec<u64> = string_digits.chars()
        .map(|c| u64::from_str(&c.to_string()).unwrap())
        .collect::<Vec<u64>>()
        .windows(span)
        .map(|w| w.into_iter().product())
        .collect();

    if let Some(x) = products.iter().max() {
        Ok(x.clone())
    } else {
        Err(String::from("Span longer than string"))
    }
}
