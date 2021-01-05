use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // a single random comma is not valid
    let _hm: HashMap<(), ()> = hashmap!(,);
}
