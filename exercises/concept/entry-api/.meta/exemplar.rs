use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counts: HashMap<&str, i32> = HashMap::with_capacity(magazine.len());

    for m in magazine {
        *counts.entry(*m).or_default() += 1;
    }

    for n in note {
        *counts.entry(*n).or_default() -= 1;

        if counts[n] < 0 {
            return false;
        }
    }

    true
}
