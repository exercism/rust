use hello_world::*;

#[test]
fn say_hi() {
    assert_eq!(hello(), "Hello, World!");
}
