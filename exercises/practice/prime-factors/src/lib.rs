pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();

    if n < 2 {
        return res;
    }

    let mut start = 2;
    let mut n: u64 = n;

    'begin: loop {
        println!("-------- In Loop --------");
        println!("n: {}, start: {}", n, start);
        for i in start..(n/2+1) {
            println!("n: {}, i: {}, n % i: {}", n, i, n % i);
            if n % i == 0 {
                res.push(i);
                n /= i;
                start = i;
                continue 'begin;
            }
        }
        res.push(n);
        break 'begin;
    }
    res
}

// pub fn factors(n: u64) -> Vec<u64> {
//     let mut res: Vec<u64> = Vec::new();

//     if n < 2 {
//         return res;
//     }

//     let mut n = n;
//     let mut i:u64 = 0;
//     let mut prime_factor = nth(i); 

//     while n > 1{
//         if n % prime_factor == 0 {
//             res.push(prime_factor);
//             n /= prime_factor;
//         }
//         else {
//             i+=1;
//             prime_factor = nth(i);
//         }
//     }
//     res
// }

// pub fn nth(n: u64) -> u64 {
//     let mut primes = Vec::new();

//     primes.push(2);

//     let mut i:u64 = 3;

//     while primes.len() < (n + 1).try_into().unwrap() {
//         if i % 2 == 0 {
//             i += 1;
//             continue;
//         }
//         if (3..=std::cmp::max(3, (i as f64).sqrt() as u64)).step_by(2)
//         .map(|c| {
//             i % c == 0 && i != c
//         })
//         .all(|c| !c) {
//             primes.push(i);
//         }
//         i += 1;
//     }
//     *primes.last().unwrap()
    
// }