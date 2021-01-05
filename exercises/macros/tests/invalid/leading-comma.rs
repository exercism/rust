use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // leading commas are not valid
    let _hm: HashMap<_, _> = hashmap!(, 'a' => 2);
}
