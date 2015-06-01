use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut output = BTreeMap::new();

    for (&n, vec) in input.iter() {
        for s in vec.iter() {
            output.insert(s.to_ascii_lowercase(), n);
        }
    }

    output
}
