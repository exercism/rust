//! A simple exercise stub might look like this:
//!
//! ```rust
//! pub fn fn_to_implement(parameter: i32) -> i32 {
//!    todo!()
//! }
//! ```
//!
//! The problem with this is that a warning will be generated because
//! `parameter` is unused. There are two obvious ways to fix this:
//!
//! 1. Prefix the parameter name with an underscore
//!
//! ```rust
//! pub fn fn_to_implement(_parameter: i32) -> i32 {
//!    todo!()
//! }
//! ```
//!
//! 2. Use the parameter in the `todo!()` macro
//!
//! ```rust
//! pub fn fn_to_implement(parameter: i32) -> i32 {
//!    todo!("use {parameter} to solve the exercise")
//! }
//! ```
//!
//! The second approach has the advantage that it disappears automatically.
//! Students will remove the macro in the process of solving the exercise.
//! The first approach requires a manual step to remove the underscore.
//! Some students don't do this (as was noticed during mentoring sessions).
//! This is ugly and even unidiomatic, because it means the compiler won't
//! warn about unused variables when that would actually be helpful.
//!
//! The second approach does have a disadvantage: it requires the parameter
//! to be `Debug`. This is usually not a problem, most of our test inputs are
//! indedd `Debug`, but it becomes a problem with generics. If a parameter is
//! generic, there must be a `Debug` bound on it. That would usually be
//! fulfilled, but it could be confusing to the students why the bound exists.
//! In that case, the first approach may be used as a fallback.
//!
//! This test makes sure the second approach is used consistently, with
//! explicitly listed exceptions.

use glob::glob;
use utils::fs::cd_into_repo_root;

static EXCEPTIONS: &[&str] = &[
    "accumulate",         // has generics (not the stub, but the solution)
    "circular-buffer",    // has generics
    "custom-set",         // has generics
    "doubly-linked-list", // has generics
    "fizzy",              // has generics
    "paasio",             // has generics
    "react",              // has generics
    "roman-numerals",     // std::fmt::Formatter is not Debug
    "simple-linked-list", // has generics
    "sublist",            // has generics
    "xorcism",            // has PhantomData
];

fn line_is_not_a_comment(line: &&str) -> bool {
    !line.trim().starts_with("//")
}

#[test]
fn no_underscore_prefixed_idents() {
    cd_into_repo_root();
    for lib_rs in glob("exercises/*/*/src/lib.rs")
        .unwrap()
        .map(Result::unwrap)
    {
        if EXCEPTIONS
            .iter()
            .any(|e| lib_rs.to_string_lossy().contains(e))
        {
            continue;
        }

        let exercise_stub = std::fs::read_to_string(&lib_rs).unwrap();

        for identifier_like in exercise_stub
            .lines()
            .filter(line_is_not_a_comment)
            // This is extremely crude parsing, but it can be improved if the need arises.
            .flat_map(|line| line.split(|c: char| !(c.is_alphabetic() || c == '_')))
        {
            if identifier_like.starts_with('_') && identifier_like != "_" {
                panic!(
                    "Exercise stub in {} contains underscore-prefixed identifier {}

    ╔════════════════════════════════════════════════════════════════╗
    ║ The use of an underscore-prefixed identifier may be justified. ║
    ║     If you think it is, add it to the list of exceptions.      ║
    ╚════════════════════════════════════════════════════════════════╝
",
                    lib_rs.display(),
                    identifier_like
                );
            }
        }
    }
}
