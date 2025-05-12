pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let candidates: Vec<u64> = (2..=upper_bound).collect();
    let nb_el: usize = (upper_bound - 1) as usize;
    let mut mask: Vec<bool> = vec![true; nb_el ]; // Same size as `candidates`. 1:1 matching. 

    // Iterate over `candidates`
    for (i, &val) in candidates.iter().enumerate() {
        if mask[i] {
            // If prime, mark as not prime all multiples of `val`
            for j in ((i+val as usize)..nb_el).step_by(val as usize) {
                mask[j] = false;
            }
        }
    }

    // Filter `candidates` based on `mask` 
    candidates
    .into_iter()
    .zip(mask)
    .filter_map(|(val, is_prime)| if is_prime { Some(val) } else { None })
    .collect()    
}
