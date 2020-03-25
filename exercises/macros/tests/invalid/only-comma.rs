use macros::hashmap;

fn main() {
    // a single random comma is not valid
    hashmap!(,);
}
