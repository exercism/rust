fn is_prime(n: u32) -> bool {
    let mut i: u32 = 3;
    while (i * i) < (n + 1) {
      if n % i == 0 {
         return false;
      }
      i += 1;
   }
   return true;
}

pub fn nth(n: u32) -> Result<u32, ()> {
    match n {
        0 => Err(()),
        1 => Ok(2),
        _ => {
            let mut count: u32 = 1;
            let mut candidate: u32 = 1;
            while count < n {
                candidate += 2;
                if is_prime(candidate) {
                    count += 1;
                }
            }
            Ok(candidate)
        }
    }
}
