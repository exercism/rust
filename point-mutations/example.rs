pub fn hamming_distance(a : &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Result::Err("inputs of different length");
    }
    
    Result::Ok(a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count())
}
