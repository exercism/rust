use macros::hashmap;

fn main() {
    // leading commas are not valid
    let _hm: ::std::collections::HashMap<_, _> = hashmap!(, 'a' => 2);
}
