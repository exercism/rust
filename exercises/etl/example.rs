use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(&n, vec)| vec.iter().map(move |c| (c.to_ascii_lowercase(), n)))
        .collect()
}
