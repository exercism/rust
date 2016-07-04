use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter().flat_map(|(&n, vec)| {
        vec.iter().map(move |s| (s.to_ascii_lowercase(), n))
    }).collect()
}
