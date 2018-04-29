pub fn two_fer(name: Option<&str>) -> String {
    match name {
        Some(name) => format!("One for {}, one for me.",name), 
        None => "One for you, one for me.".to_string(),
    }
}
