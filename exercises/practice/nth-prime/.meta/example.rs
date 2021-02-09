fn is_prime(n: u32) -> bool {
    let mut i = 3;
    while (i * i) < (n + 1) {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        2
    } else {
        let mut count = 0;
        let mut candidate = 1;
        while count < n {
            candidate += 2;
            if is_prime(candidate) {
                count += 1;
            }
        }
        candidate
    }
}
