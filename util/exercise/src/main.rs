use clap::{App, Arg, ArgMatches, SubCommand};
use exercise::{
    cmd::{configure, fetch_configlet, generate, update},
    errors::Result,
};
use failure::format_err;

// Creates a new CLI app with appropriate matches
// and returns the initialized matches.
fn init_app<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates new exercise")
                .arg(Arg::with_name("exercise_name").required(true).help("The name of the generated exercise"))
                .arg(Arg::with_name("configure").long("configure").short("c").help(
                    "If set, the command will edit the config.json file after generating the exercise",
                ))
                .arg(Arg::with_name("use_maplit").long("use-maplit").short("m").help("Use the maplit crate to improve the readability of the generated test suite")),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates the specified exercise")
                .arg(Arg::with_name("exercise_name").help("The name of the updated exercise"))
                .arg(Arg::with_name("use_maplit").long("use-maplit").short("m").help("Use the maplit crate to improve the readability of the updated test suite"))
                .arg(Arg::with_name("dont_update_readme").long("dont-update-readme").short("r").help("If set, the README of the exercise would not be updated"))
                .arg(Arg::with_name("configure").long("configure").short("c").help(
                    "If set, the command will edit the config.json file after updating the exercise",
                ))
        )
        .subcommand(
            SubCommand::with_name("configure")
                .about("Edits config.json for the specified exercise")
                .arg(Arg::with_name("exercise_name").required(true).help("The name of the configured exercise")),
        )
        .subcommand(
            SubCommand::with_name("fetch_configlet")
                .about("Downloads and extracts configlet utility into the repo's /bin directory")
        )
        .get_matches()
}

// Determine which subcommand was used
// and call the appropriate function.
fn process_matches(matches: &ArgMatches<'_>) -> Result<()> {
    match matches.subcommand() {
        ("generate", Some(generate_matches)) => {
            let exercise_name = generate_matches
                .value_of("exercise_name")
                .ok_or_else(|| format_err!("exercise name not present in args"))?;
            let run_configure = generate_matches.is_present("configure");
            let use_maplit = generate_matches.is_present("use_maplit");

            generate::generate_exercise(exercise_name, use_maplit)?;

            if run_configure {
                configure::configure_exercise(exercise_name)?;
            }
        }

        ("update", Some(update_matches)) => {
            let exercise_name = update_matches
                .value_of("exercise_name")
                .ok_or_else(|| format_err!("exercise name not present in args"))?;

            let run_configure = update_matches.is_present("configure");

            let use_maplit = update_matches.is_present("use_maplit");

            let update_readme = !update_matches.is_present("dont_update_readme");

            update::update_exercise(exercise_name, use_maplit)?;

            if run_configure {
                configure::configure_exercise(exercise_name)?;
            }

            if update_readme {
                exercise::run_configlet_command("generate", &[".", "-o", exercise_name])?;
            }
        }

        ("configure", Some(configure_matches)) => {
            configure::configure_exercise(
                configure_matches
                    .value_of("exercise_name")
                    .ok_or_else(|| format_err!("exercise name not present in args"))?,
            )?;
        }

        ("fetch_configlet", Some(_fetch_configlet_matches)) => {
            if let Ok(fetch_path) = fetch_configlet::download() {
                println!(
                    "Downloaded and moved the configlet utility to the {:?} path",
                    fetch_path
                );
            } else {
                println!("Failed to fetch the configlet utility");
            }
        }

        ("", None) => {
            println!("No subcommand was used.\nUse exercise --help to learn about the possible subcommands.");
        }

        _ => unreachable!(),
    };

    Ok(())
}

fn main() -> Result<()> {
    let matches = init_app();

    process_matches(&matches)?;
    Ok(())
}
