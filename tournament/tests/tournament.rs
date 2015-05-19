use std::fs::File;
use std::path::Path;
use std::io::Read;

extern crate tournament;

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
fn test_good() {
    assert_eq!(tournament::tally(&Path::new("tests/input1.txt"), &Path::new("tests/output1.txt")).unwrap(), 6);
    file_equal("tests/output1.txt", "tests/expected1.txt");
}

#[test]
#[ignore]
fn test_ignore_bad_lines() {
    assert_eq!(tournament::tally(&Path::new("tests/input2.txt"), &Path::new("tests/output2.txt")).unwrap(), 6);
    file_equal("tests/output2.txt", "tests/expected2.txt");
}

#[test]
#[ignore]
fn test_incomplete_competition() {
    assert_eq!(tournament::tally(&Path::new("tests/input3.txt"), &Path::new("tests/output3.txt")).unwrap(), 4);
    file_equal("tests/output3.txt", "tests/expected3.txt");
}
