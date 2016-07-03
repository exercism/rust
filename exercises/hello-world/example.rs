pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Hello, World!".to_string(),
    }
}
