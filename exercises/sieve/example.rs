pub fn primes_up_to(limit: i32) -> Vec<i32> {
    let mut integers = (1..limit).map(|x| x + 1).collect::<Vec<i32>>();
    let mut p = Some(2);

    while let Some(y) = p {
        integers.retain(|&x| (x == y) || (x % y != 0));
        p = integers.clone().into_iter().find(|x| *x > y);
    }
    integers
}
