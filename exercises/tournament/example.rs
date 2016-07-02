use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn tally(input_filename: &Path, _: &Path) -> Result<u32> {
    let reader = BufReader::with_capacity(2048, File::open(input_filename).unwrap());
    let mut count = 0;
    for line in reader.lines() {    
        let line = line.unwrap();
        let parts: Vec<&str> = line.trim_right().split(';').collect(); 
        if parts.len() != 3 { continue; }
        let outcome = parts[2];
        match outcome {
            "win" => {
                count += 1;
            },
            "draw" => {
                count += 1;
            },
            "loss" => {
                count += 1;
            },
            _ => () // Ignore bad lines
        }
    }
    Ok(count)
}
