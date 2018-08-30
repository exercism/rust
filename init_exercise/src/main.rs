extern crate clap;
mod cmd;

use clap::{App, Arg, ArgMatches, SubCommand};
use cmd::{configure, generate, update};

// Creates a new CLI app with appropriate matches
// and returns the initialized matches.
fn init_app<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates new exercise")
                .arg(Arg::with_name("exercise_name").help("The name of the generated exercise"))
                .arg(Arg::with_name("no_configure").long("no-configure").short("n").help(
                    "If set, the command will not edit config.json after generating the exercise",
                ))
                .arg(Arg::with_name("use_maplit").long("use-maplit").short("m").help("Use the maplit crate to improve the readability of the generated test suite")),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates the specified exercise")
                .arg(Arg::with_name("exercise_name").help("The name of the updated exercise"))
                .arg(Arg::with_name("no_configure").long("no-configure").short("n").help(
                    "If set, the command will not edit config.json after updating the exercise",
                ))
        )
        .subcommand(
            SubCommand::with_name("configure")
                .about("Edits config.json for the specified exercise")
                .arg(Arg::with_name("exercise_name").help("The name of the configured exercise")),
        )
        .get_matches()
}

// Determine which subcommand was used
// and call the appropriate function.
fn process_matches(matches: &ArgMatches) {
    match matches.subcommand() {
        ("generate", Some(generate_matches)) => generate::process_matches(&generate_matches),
        ("update", Some(update_matches)) => println!("Update!"),
        ("configure", Some(configure_matches)) => println!("Configure!"),
        ("", None) => {
            println!("No subcommand was used.\nUse init_exercise --help to learn about the possible subcommands.")
        }
        _ => unreachable!(),
    }
}

fn main() {
    let matches = init_app();

    process_matches(&matches);
}
