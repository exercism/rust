use macros::hashmap;

fn main() {
    // three arguments are invalid
    hashmap!('a' => 1, 'b');
}
