/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {

    let mut res = 0;
    
    if s1.chars().count() != s2.chars().count() {
        return None;
    }
    else {
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                res += 1;
            }
        }
    }
    Some(res)

}
