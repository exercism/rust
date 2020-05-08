use macros::hashmap;

fn main() {
    // a single random arrow is not valid
    let _hm: ::std::collections::HashMap<(), ()> = hashmap!(=>);
}
