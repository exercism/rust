pub fn hamming_distance(a: &str, b: &str) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }

    Some(a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count())
}
