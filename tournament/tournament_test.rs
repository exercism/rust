#![crate_name = "tournament_test"]
#![crate_type = "lib"]

use std::io::File;

mod tournament;

#[test]
#[ignore]
fn test_good() {
    assert_eq!(tournament::tally(&Path::new("input1.txt"), &Path::new("output1.txt")), Ok(6));
    let output = match File::open(&Path::new("output1.txt")).read_to_string() {
        Err(e) => fail!("Couldn't open output1.txt: {}", e),
        Ok(s) => s
    };
    let expected = match File::open(&Path::new("expected1.txt")).read_to_string() {
        Err(e) => fail!("Couldn't open expected1.txt, something is wrong with exercism: {}", e),
        Ok(s) => s
    };
    // (Initial newlines make for better readable test failures.)
    assert_eq!("\n".to_string() + output, "\n".to_string() + expected);
}

#[test]
#[ignore]
fn test_ignore_bad_lines() {
    assert_eq!(tournament::tally(&Path::new("input2.txt"), &Path::new("output2.txt")), Ok(6));
    let output = match File::open(&Path::new("output2.txt")).read_to_string() {
        Err(e) => fail!("Couldn't open output1.txt: {}", e),
        Ok(s) => s
    };
    let expected = match File::open(&Path::new("expected2.txt")).read_to_string() {
        Err(e) => fail!("Couldn't open expected1.txt, something is wrong with exercism: {}", e),
        Ok(s) => s
    };
    assert_eq!("\n".to_string() + output, "\n".to_string() + expected);
}

#[test]
#[ignore]
fn test_incomplete_competition() {
    assert_eq!(tournament::tally(&Path::new("input3.txt"), &Path::new("output3.txt")), Ok(4));
    let output = match File::open(&Path::new("output3.txt")).read_to_string() {
        Err(e) => fail!("Couldn't open output1.txt: {}", e),
        Ok(s) => s
    };
    let expected = match File::open(&Path::new("expected3.txt")).read_to_string() {
        Err(e) => fail!("Couldn't open expected1.txt, something is wrong with exercism: {}", e),
        Ok(s) => s
    };
    assert_eq!("\n".to_string() + output, "\n".to_string() + expected);
}
