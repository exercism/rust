use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
mod utils;
use utils::escape_double_quotes::*;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for line in reader.lines() {
        let input = line?;
        let output = escape_double_quotes(&input);
        writeln!(writer, "{}", output)?;
    }

    Ok(())
}

