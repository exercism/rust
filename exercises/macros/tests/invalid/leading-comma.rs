use macros::hashmap;

fn main() {
    // leading commas are not valid
    hashmap!(, 'a' => 2);
}
