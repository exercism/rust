use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // Key value pairs must be separated by commas
    let _hm: HashMap<_, _> = hashmap!('a' => 1 'b' => 2);
}
