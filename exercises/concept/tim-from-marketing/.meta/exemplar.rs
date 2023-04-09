pub fn print_name_badge(id: Option<u8>, name: &str, department: Option<&str>) -> String {
    let mut works_at = "OWNER".to_string();

    if department != None {
        works_at = department.unwrap().to_uppercase();
    }

    if id == None {
        format!("{} - {}", name, works_at)
    } else {
        format!("[{}] - {} - {}", id.unwrap(), name, works_at)
    }
}
