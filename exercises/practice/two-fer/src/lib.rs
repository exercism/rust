pub fn twofer(name: &str) -> String {
    match name {
        "" => "One for you, one for me.".to_string(),
        _ => format!("One for {}, one for me.", name),
    }
}
