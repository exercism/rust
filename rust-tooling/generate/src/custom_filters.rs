use tera::{Kwargs, Number, State, TeraResult as Result, Value};

pub fn to_hex(value: u64, _args: Kwargs, _state: &State) -> Result<Value> {
    Ok(Value::safe_string(&format!("{:x}", value)))
}

pub fn make_ident(value: &str, _args: Kwargs, _state: &State) -> Result<Value> {
    let value = split_camel_case_with_underscore(value);
    let value = slug::slugify(value).replace('-', "_");
    if !value.chars().next().unwrap_or_default().is_alphabetic() {
        // identifiers cannot start with digits etc.
        return Ok(Value::safe_string(&format!("test_{value}")));
    }
    Ok(Value::safe_string(&value))
}

fn split_camel_case_with_underscore(input: &str) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    let mut i = chars.len() - 1;
    while i > 0 {
        let (left, right) = (chars[i - 1], chars[i]);
        if left.is_ascii_lowercase() && right.is_ascii_uppercase() {
            chars.insert(i, '_');
        }
        i -= 1;
    }
    chars.into_iter().collect()
}

pub fn fmt_num(value: Number, _args: Kwargs, _state: &State) -> Result<Value> {
    let mut num: Vec<_> = value.to_string().into();
    num.reverse();

    let mut pretty_digits = num
        .chunks(3)
        .flat_map(|digits| digits.iter().copied().chain(*b"_"))
        .collect::<Vec<_>>();
    if pretty_digits.last() == Some(&b'_') {
        pretty_digits.pop();
    }
    pretty_digits.reverse();
    let pretty_num = String::from_utf8(pretty_digits).unwrap_or_default();
    Ok(Value::safe_string(&pretty_num))
}
