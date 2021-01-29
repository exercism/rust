use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // a single argument is invalid
    let _hm: HashMap<_, _> = hashmap!('a');
}
