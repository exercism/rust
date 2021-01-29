use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // using only commas is invalid
    let _hm: HashMap<_, _> = hashmap!('a', 1);
}
