use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // three arguments are invalid
    hashmap!('a' => 1, 'b');
}
