use rand::Rng;

pub fn encode_random(s: &str) -> (String, String) {
    let mut r = rand::thread_rng();
    let mut key = String::new();
    for _ in 0..100 {
        key.push(char::from(b'a' + r.gen_range(0..26)));
    }
    let encoded = encode(&key, s);
    (key, encoded.unwrap())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    shift(key, s, 1)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    shift(key, s, -1)
}

fn shift(key: &str, s: &str, dir: i8) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut o = String::new();
    let mut i = 0;
    let mut key_arr = Vec::new();
    for c in key.chars() {
        if !c.is_ascii_lowercase() {
            return None;
        }
        key_arr.push(c);
    }
    for c in s.chars() {
        let shift = key_arr[i % key_arr.len()] as i8 - 'a' as i8;
        let n = ((c as i8 - b'a' as i8 + dir * shift) % 26 + 26) % 26;
        o.push(char::from(b'a' + n as u8));
        i += 1;
    }
    Some(o)
}
