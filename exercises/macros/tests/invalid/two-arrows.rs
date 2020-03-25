use macros::hashmap;

fn main() {
    // a trailing => isn't valid either
    hashmap!('a' => 2, =>);
}
