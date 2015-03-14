#![feature(collections)]

#![crate_name = "tournament_test"]
#![crate_type = "lib"]

#![feature(old_path)]
#![feature(io)]
#![feature(std_misc)]
#![feature(core)]

use std::fs::File;

mod tournament;

#[test]
#[ignore]
fn test_good() {
    assert_eq!(tournament::tally(&Path::new("input1.txt"), &Path::new("output1.txt")), Ok(6));
    let output = match File::open(&Path::new("output1.txt")).unwrap().read_to_string() {
        Err(e) => panic!("Couldn't open output1.txt: {}", e),
        Ok(s) => s
    };
    let expected = match File::open(&Path::new("expected1.txt")).unwrap().read_to_string() {
        Err(e) => panic!("Couldn't open expected1.txt, something is wrong with exercism: {}", e),
        Ok(s) => s
    };
    // (Initial newlines make for better readable test panics.)
    assert_eq!("\n".to_string() + output.as_slice(), "\n".to_string() + expected.as_slice());
}

#[test]
#[ignore]
fn test_ignore_bad_lines() {
    assert_eq!(tournament::tally(&Path::new("input2.txt"), &Path::new("output2.txt")), Ok(6));
    let output = match File::open(&Path::new("output2.txt")).unwrap().read_to_string() {
        Err(e) => panic!("Couldn't open output1.txt: {}", e),
        Ok(s) => s
    };
    let expected = match File::open(&Path::new("expected2.txt")).unwrap().read_to_string() {
        Err(e) => panic!("Couldn't open expected1.txt, something is wrong with exercism: {}", e),
        Ok(s) => s
    };
    assert_eq!("\n".to_string() + output.as_slice(), "\n".to_string() + expected.as_slice());
}

#[test]
#[ignore]
fn test_incomplete_competition() {
    assert_eq!(tournament::tally(&Path::new("input3.txt"), &Path::new("output3.txt")), Ok(4));
    let output = match File::open(&Path::new("output3.txt")).unwrap().read_to_string() {
        Err(e) => panic!("Couldn't open output1.txt: {}", e),
        Ok(s) => s
    };
    let expected = match File::open(&Path::new("expected3.txt")).unwrap().read_to_string() {
        Err(e) => panic!("Couldn't open expected1.txt, something is wrong with exercism: {}", e),
        Ok(s) => s
    };
    assert_eq!("\n".to_string() + output.as_slice(), "\n".to_string() + expected.as_slice());
}
