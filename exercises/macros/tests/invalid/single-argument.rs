use macros::hashmap;

fn main() {
    // a single argument is invalid
    let _hm: ::std::collections::HashMap<_, _> = hashmap!('a');
}
