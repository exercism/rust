pub fn nth(n: u32) -> Option<u32> {
	let upper_limit;
	if n <= 0 {
		return None;
	}
	// is known that if n>6, the nth prime number is less than n ln(n) + n ln(ln(n))
	if n > 6 {
		let m = n as f32;
		upper_limit =  m * (m.ln() + (m.ln()).ln());
		println!("{}",upper_limit);
	} else {
		upper_limit = 13.0; // the 6th prime number
	}
	
	let primes: Vec<bool> = eratosthenes(upper_limit as usize);
	let mut i:u32 = 0;
	let mut j:u32 = 0;
	while j<n {
		if primes[i as usize] {
			j+=1;
		}
		i+=1;
	}
	Some(i -1)
}

fn eratosthenes(n: usize) -> Vec<bool> {
	let mut v= vec![];
	let n =n+1;
	v.push(false);
	v.push(false);
	for _ in 2..n { v.push(true); }
	for i in 2..n {
		if v[i] {
			let mut j = i*i;
			while j < n {
				v[j] = false;
				j += i;
			}
		}
	}
	v
}