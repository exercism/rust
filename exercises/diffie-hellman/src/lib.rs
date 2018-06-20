pub fn private_key(p: u64) -> u64 {
    unimplemented!("Pick a private key greater than 1 and less than {}", p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate public key using prime numbers {} and {}, and private key {}",
        p,
        g,
        a
    )
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate secret key using prime number {}, public key {}, and private key {}",
        p,
        b_pub,
        a
    )
}
