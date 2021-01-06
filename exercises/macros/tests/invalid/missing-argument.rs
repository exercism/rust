use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // an argument should come between each pair of commas
    let _hm: HashMap<_, _> = hashmap!('a' => 1, , 'b' => 2);
}
