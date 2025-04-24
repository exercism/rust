pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();

    primes.push(2);

    let mut i:u32 = 3;

    while primes.len() < (n + 1).try_into().unwrap() {
        if i % 2 == 0 {
            i += 1;
            continue;
        }
        if (3..=std::cmp::max(3, (i as f64).sqrt() as u32)).step_by(2)
        .map(|c| {
            i % c == 0 && i != c
        })
        .all(|c| !c) {
            primes.push(i);
        }
        i += 1;
    }
    *primes.last().unwrap()
    
}
