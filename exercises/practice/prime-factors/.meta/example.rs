pub fn factors(n: u64) -> Vec<u64> {
    let mut val = n;
    let mut out: Vec<u64> = vec![];
    let mut possible: u64 = 2;
    while val > 1 {
        while val.is_multiple_of(possible) {
            out.push(possible);
            val /= possible;
        }
        possible += 1;
    }
    out
}
