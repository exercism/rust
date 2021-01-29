// The &'static here means the return type has a static lifetime.
// This is a Rust feature that you don't need to worry about now.
pub fn hello() -> &'static str {
    "Goodbye, World!"
}
