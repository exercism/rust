use macros::hashmap;

fn main() {
    // a single trailing comma is okay, but two is not
    let _hm: ::std::collections::HashMap<_, _> = hashmap!('a' => 2, ,);
}
