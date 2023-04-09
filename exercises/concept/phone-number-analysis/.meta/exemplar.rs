pub fn analyze(phone_number: &str) -> (bool, bool, &str) {
    let (dial_code, number) = phone_number.split_at(3);

    let (prefix_code, last_4_with_dash) = number.split_at(4);

    let (_, last_4) = last_4_with_dash.split_at(1);

    (dial_code == "212", prefix_code == "-555", last_4)
}

pub fn is_fake(info: (bool, bool, &str)) -> bool {
    info.1
}
