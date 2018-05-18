pub fn raindrops(n: usize) -> String {
	let mut stg = String::new();

    if n % 3 == 0 {
    	stg.push_str("Pling");
    }
    if n % 5 == 0 {
    	stg.push_str("Plang");
    }
    if n % 7 == 0 {
    	stg.push_str("Plong");
    }
    if stg.is_empty() {
    	stg = n.to_string();
    }
    stg
}
