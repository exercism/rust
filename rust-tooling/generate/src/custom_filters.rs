use std::collections::HashMap;

use tera::{Result, Value};

type Filter = fn(&Value, &HashMap<String, Value>) -> Result<Value>;

pub static CUSTOM_FILTERS: &[(&str, Filter)] = &[("to_hex", to_hex), ("snake_case", snake_case)];

pub fn to_hex(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    Ok(serde_json::Value::String(format!(
        "{:x}",
        value.as_u64().unwrap()
    )))
}

pub fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    Ok(serde_json::Value::String(
        // slug is the same dependency tera uses for its builtin 'slugify'
        slug::slugify(value.as_str().unwrap()).replace('-', "_"),
    ))
}
