extern crate clap;
extern crate exercise;
extern crate failure;
extern crate lazy_static;
extern crate regex;
extern crate syn;

use clap::clap_app;
use exercise::paths;
use failure::Error;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use syn::{visit::Visit, ItemFn};

lazy_static! {
    static ref process_case_fn: Regex = RegexBuilder::new(r"^process_[a-z0-9_]*_case$")
        .case_insensitive(true)
        .build()
        .expect("process_case_fn regex must compile");
}

fn is_test(func: &ItemFn) -> bool {
    func.attrs.iter().any(|attr| {
        attr.path.segments.len() == 1 && attr.path.segments[0].ident.to_string() == "test"
    })
}

#[derive(Default)]
struct TestFuncs<'ast> {
    processors: Vec<&'ast ItemFn>,
    cases: Vec<&'ast ItemFn>,
}

impl<'ast> Visit<'ast> for TestFuncs<'ast> {
    fn visit_item_fn(&mut self, func: &'ast ItemFn) {
        if is_test(func) {
            self.cases.push(func);
        } else if process_case_fn.is_match(&func.ident.to_string()) {
            self.processors.push(func);
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = clap_app!(syn_test =>
        (about: "test app for figuring out how to properly use Rust's syn crate")
        (@arg EXERCISE: +required "select exercise")
        (@arg ast: --ast "print the exercise tests' AST")
        (@arg nopath: -P --nopath "do not print the path to the test file")
        (@arg noheaders: -H --noheaders "do not emit headers")
        (@arg processors: -p --processors "emit case processors")
        (@arg cases: -c --cases "emit test cases")
    )
    .get_matches();
    let exercise = matches.value_of("EXERCISE").unwrap(); // clap ensures it exists
    let test_path = paths::tests(exercise);
    if !matches.is_present("nopath") {
        println!("path: {}", test_path.display());
    }

    // can't just use ? to get this value because syn Errors are not thread-
    // safe, apparently.
    let test_file = {
        match syn::parse_file(&std::fs::read_to_string(test_path)?) {
            Ok(test_file) => test_file,
            Err(e) => {
                eprintln!("failed to parse test file: {}", e);
                std::process::exit(1);
            }
        }
    };
    if matches.is_present("ast") {
        println!("{:#?}", test_file);
    }

    let funcs = {
        let mut fs = TestFuncs::default();
        fs.visit_file(&test_file);
        fs
    };
    if matches.is_present("processors") {
        if !matches.is_present("noheaders") {
            println!("Case processors:");
        }
        for processor in funcs.processors.iter() {
            println!("  {}", processor.ident.to_string(),);
        }
    }

    if matches.is_present("cases") {
        if !matches.is_present("noheaders") {
            println!("Test cases:");
        }
        for case in funcs.cases.iter() {
            println!("  {}", case.ident.to_string(),);
        }
    }
    Ok(())
}
