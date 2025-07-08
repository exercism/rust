use rand::{rng, Rng};


fn is_key_invalid(key: &str) -> bool {
    
    if key.len() == 0 {
        return true;
    } 
    
    key
    .chars()
    .any(|c| !c.is_alphabetic() || !c.is_ascii_lowercase() || !c.is_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if is_key_invalid(key) {
        return None;
    }

    let mut res = String::new();
    for (c, k) in s.chars().zip(key.chars().cycle()) {
        println!("encode {c}, {k}");
        println!("encode : {}, {}", c as u8, k as u8);
        println!("{}", c as u8 + k as u8 - 'a' as u8);
        // res.push(('a' as u8  + (26 + k as u8 - c as u8) % 26 ) as char);
        // res.push((((k as u8 - 'a' as u8) + c as u8) % 26 ) as char);
        res.push(('a' as u8 + (k as u8 - 'a' as u8 + c as u8 - 'a' as u8) % 26) as char);
    }

    println!("{}", res);

    Some(res)
}

pub fn decode(key: &str, s: &str) -> Option<String> {

    if is_key_invalid(key) {
        return None;
    }

    let mut res = String::new();
    for (c, k) in s.chars().zip(key.chars().cycle()) {
        println!("decode : {c}, {k}");
        println!("decode : {}, {}", c as u8, k as u8);
        res.push(('a' as u8 + (26 + c as u8 - k as u8) % 26 ) as char)
    }
    
    println!("{}", res);

    Some(res)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rng();
    let key: String = (0..100).map(|_| {
        ('a' as u8 + rng.random_range(0..26)) as char
    })
    .collect();

    let encoded = encode(&key, s);
    (key, encoded.unwrap())
}
