use std::fs::File;
use std::path::Path;
use std::io::Read;

extern crate transpose;

#[test]
fn single_line() {
    let input = read_to_string("tests/input-1.txt");
    let output = read_to_string("tests/output-1.txt");
    assert_eq!(output, transpose::transpose(input));
}

#[test]
#[ignore]
fn two_lines() {
    let input = read_to_string("tests/input-2.txt");
    let output = read_to_string("tests/output-2.txt");
    assert_eq!(output, transpose::transpose(input));
}

#[test]
#[ignore]
fn these_tests() {
    let input = read_to_string("tests/input-3.txt");
    let output = read_to_string("tests/output-3.txt");
    assert_eq!(output, transpose::transpose(input));
}

fn read_to_string(filename: &str) -> String {
    let file = File::open(&Path::new(filename));
    match file {
        Err(e) => panic!("Couldn't open {}: {}", filename, e),
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Err(e) => panic!("Couldn't read {}: {}", filename, e),
                Ok(_) => buf
            }
        }
    }
}

