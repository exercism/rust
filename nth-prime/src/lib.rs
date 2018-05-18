pub fn nth(n: u32) -> Option<u32> {
	if n == 0 {
		return None;
	}
	let limit;
	// is known that if n=>6, the nth prime number is less than n * (n * n.ln()).ln()
	if n >= 6 {
		let n = n as f32;
		limit =  (n * (n * n.ln()).ln()) as usize;
	} else {
		limit = 11; // the 5th prime number
	}
	let primes = eratosthenes(limit);
	let mut j = 1;
	let mut i = 2; // the jth prime number 
	while j<=n {
		if primes[i] { j+=1; }
		i+=1;
	}
	Some(i as u32 - 1)
}

/* the sieve of Eratosthenes is a simple, ancient algorithm
   for finding all prime numbers up to any given limit. */
fn eratosthenes(limit: usize) -> Vec<bool> {
	let mut v = vec![false,false];
	for _ in 2..(limit+1) { v.push(true); }
	for i in 2..(limit+1) {
		if v[i] {
			let mut j = i*i;
			while j < limit {
				v[j] = false;
				j += i;
			}
		}
	}
	v
}