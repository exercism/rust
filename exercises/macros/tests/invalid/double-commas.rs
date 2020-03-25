use macros::hashmap;

fn main() {
    // a single trailing comma is okay, but two is not
    hashmap!('a' => 2, ,);
}
