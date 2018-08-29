extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates new exercise")
                .arg(Arg::with_name("exercise_name").help("The name of the generated exercise"))
                .arg(Arg::with_name("dont_update_config").long("dont-update-config").short("d").help(
                    "If set, the command will not update config.json after generating the exercise",
                ))
                .arg(Arg::with_name("use_maplit").long("use-maplit").short("m").help("Use the maplit crate to improve the readability of the generated test suite")),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates the specified exercise")
                .arg(Arg::with_name("exercise_name").help("The name of the updated exercise")),
        )
        .get_matches();
}
