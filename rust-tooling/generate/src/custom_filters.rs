use std::collections::HashMap;

use tera::{Result, Value};

type Filter = fn(&Value, &HashMap<String, Value>) -> Result<Value>;

pub static CUSTOM_FILTERS: &[(&str, Filter)] = &[
    ("to_hex", to_hex),
    ("make_ident", make_ident),
    ("fmt_num", fmt_num),
];

pub fn to_hex(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let Some(value) = value.as_u64() else {
        return Err(tera::Error::call_filter(
            "to_hex filter expects an unsigned integer",
            "serde_json::value::Value::as_u64",
        ));
    };
    Ok(serde_json::Value::String(format!("{:x}", value)))
}

pub fn make_ident(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let Some(value) = value.as_str() else {
        return Err(tera::Error::call_filter(
            "make_ident filter expects a string",
            "serde_json::value::Value::as_str",
        ));
    };
    let value = slug::slugify(value).replace('-', "_");
    if !value.chars().next().unwrap_or_default().is_alphabetic() {
        // identifiers cannot start with digits etc.
        return Ok(Value::String(format!("test_{value}")));
    }
    Ok(Value::String(value))
}

pub fn fmt_num(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let Some(value) = value.as_number() else {
        return Err(tera::Error::call_filter(
            "fmt_num filter expects a number",
            "serde_json::value::Value::as_number",
        ));
    };
    let mut num: Vec<_> = value.to_string().into();
    num.reverse();

    let mut pretty_digits = num
        .chunks(3)
        .flat_map(|digits| digits.iter().copied().chain([b'_']))
        .collect::<Vec<_>>();
    if pretty_digits.last() == Some(&b'_') {
        pretty_digits.pop();
    }
    pretty_digits.reverse();
    let pretty_num = String::from_utf8(pretty_digits).unwrap_or_default();
    Ok(Value::String(pretty_num))
}
