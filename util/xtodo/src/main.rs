extern crate clap;
extern crate reqwest;
extern crate serde_json;

mod cmd;

use clap::{App, ArgMatches, SubCommand};

fn init_app<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("missing")
                .about("List all unimplemented exercises on the current track"),
        ).get_matches()
}

fn process_matches(matches: &ArgMatches) {
    match matches.subcommand() {
        ("missing", _) => {
            cmd::list_missing_exercises();
        }

        ("", _) => {
            println!("No subcommand was used.\nUse 'xtodo help' to learn about the possible subcommands.");
        }

        _ => unreachable!(),
    }
}

fn main() {
    let matches = init_app();

    process_matches(&matches);
}
