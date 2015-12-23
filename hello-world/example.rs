pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(ref n) => format!("Hello, {}!", n),
        None => "Hello, World!".to_string(),
    }
}
