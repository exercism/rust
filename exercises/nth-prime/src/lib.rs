
pub fn nth(n: u32) -> u32 {
    let mut index = 2;
    let mut counter  = 0;
    let result = loop {
        if is_prime(index) {
            if counter == n {
                break index;
            }
            counter += 1;
        }
        index += 1;
    };
    return result;
}
fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 ==0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while (i * i ) <= n {
        if n % i == 0 || n % (i+2) == 0 {
            return false;
        }
        i += 6
    }

    return true;
}
