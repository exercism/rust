use macros::hashmap;
use std::collections::HashMap;

fn main() {
    // a trailing => isn't valid either
    hashmap!('a' => 2, =>);
}
