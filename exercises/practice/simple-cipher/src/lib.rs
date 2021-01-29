pub fn encode(key: &str, s: &str) -> Option<String> {
    unimplemented!("Use {} to encode {} using shift cipher", key, s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    unimplemented!("Use {} to decode {} using shift cipher", key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
        s
    )
}
