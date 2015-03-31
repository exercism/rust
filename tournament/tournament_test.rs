#![crate_name = "tournament_test"]
#![crate_type = "lib"]

#![feature(slice_patterns)]
#![feature(convert)]

use std::fs::File;
use std::path::Path;
use std::io::Read;

mod tournament;

fn file_equal(output_file: &str, expected_file: &str) {
    let output = match File::open(&Path::new(output_file)) {
        Err(e) => panic!("Couldn't open {}: {}", output_file, e),
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Err(e) => panic!("Couldn't read {}: {}", output_file, e),
                Ok(_) => buf
            }
        }
    };
    let expected = match File::open(&Path::new(expected_file)) {
        Err(e) => panic!("Couldn't open {}: {}", expected_file, e),
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Err(e) => panic!("Couldn't read {}: {}", expected_file, e),
                Ok(_) => buf
            }
        }
    };
    assert_eq!("\n".to_string() + output.as_ref(), "\n".to_string() + expected.as_ref());
    
}


#[test]
#[ignore]
fn test_good() {
    assert_eq!(tournament::tally(&Path::new("input1.txt"), &Path::new("output1.txt")), Ok(6));
    file_equal("output1.txt", "expected1.txt");
}

#[test]
#[ignore]
fn test_ignore_bad_lines() {
    assert_eq!(tournament::tally(&Path::new("input2.txt"), &Path::new("output2.txt")), Ok(6));
    file_equal("output2.txt", "expected2.txt");
}

#[test]
#[ignore]
fn test_incomplete_competition() {
    assert_eq!(tournament::tally(&Path::new("input3.txt"), &Path::new("output3.txt")), Ok(4));
    file_equal("output3.txt", "expected3.txt");
}
