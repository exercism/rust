use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // a single random arrow is not valid
    let _hm: HashMap<(), ()> = hashmap!(=>);
}
