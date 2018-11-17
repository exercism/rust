use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..sum {
        /*
         * (where n is a one-character alias for sum)
         * c = n - a - b
         * a^2 + b^2 = c^2
         * a^2 + b^2 = n^2 - 2an - 2bn + a^2 + 2ab + b^2
         * 2bn - 2ab = n^2 - 2an
         * 2b(n - a) = n(n-2a)
         * b = n(n-2a) / 2(n-a)
         * b = (n(n-a) - an) / 2(n-a)
         */
        let b = sum / 2 - a * sum / (2 * (sum - a));
        if a >= b {
            break;
        }
        let c = sum - (a + b);

        if a * a + b * b == c * c {
            triplets.insert([a, b, c]);
        }
    }

    triplets
}
