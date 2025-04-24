pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut n = n;
    let mut counter = 0;

    while n != 1 {
        match n % 2 == 0 {
            true => {
                n /= 2;
            },
            false => {
                n = n*3 + 1;
            }

        }
        counter +=1 ;
    }
    Some(counter)
}
