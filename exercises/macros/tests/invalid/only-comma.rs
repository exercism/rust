use macros::hashmap;

fn main() {
    // a single random comma is not valid
    let _hm: ::std::collections::HashMap<(), ()> = hashmap!(,);
}
