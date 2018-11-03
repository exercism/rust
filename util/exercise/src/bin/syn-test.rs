extern crate clap;
extern crate exercise;
extern crate failure;
extern crate syn;

use clap::clap_app;
use exercise::paths;
use failure::Error;

fn main() -> Result<(), Error> {
    let matches = clap_app!(syn_test =>
        (@arg EXERCISE: +required "select exercise")
        (@arg ast: --ast "print the exercise tests' AST")
    )
    .get_matches();
    let exercise = matches.value_of("EXERCISE").unwrap(); // clap ensures it exists
    let test_path = paths::tests(exercise);
    println!("path: {}", test_path.display());
    match syn::parse_file(&std::fs::read_to_string(test_path)?) {
        Ok(test_file) => {
            if matches.is_present("ast") {
                println!("{:#?}", test_file);
            }
        }
        Err(e) => {
            eprintln!("failed to parse test file: {}", e);
        }
    }
    Ok(())
}
