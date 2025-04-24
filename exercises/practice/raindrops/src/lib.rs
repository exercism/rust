pub fn raindrops(n: u32) -> String {
    let mut res = String::new();

    if n % 3 == 0 {
        res.push_str("Pling");
    }
    if n % 5 == 0 {
        res.push_str("Plang");
    }
    if n % 7 == 0 {
        res.push_str("Plong");
    }

    if res.is_empty() {
        format!("{}", n)
    }
    else {
        res
    }
}
