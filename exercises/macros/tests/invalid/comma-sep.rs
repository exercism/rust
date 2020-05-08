use macros::hashmap;

fn main() {
    // using only commas is invalid
    let _hm: ::std::collections::HashMap<_, _> = hashmap!('a', 1);
}
