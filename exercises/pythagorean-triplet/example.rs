pub fn find(sum: u32) -> Vec<[u32; 3]> {
    let mut triplets = vec![];

    for a in 1..sum {
        for b in (a + 1)..(sum - a) {
            let c = sum - (a + b);

            if a * a + b * b == c * c {
                triplets.push([a, b, c]);
            }
        }
    }

    triplets
}
