use std::path::PathBuf;

use clap::Parser;
use cli::{AddArgs, FullAddArgs, UpdateArgs};
use models::track_config::{self, TRACK_CONFIG};

mod cli;
mod exercise_generation;

fn main() {
    utils::fs::cd_into_repo_root();

    let cli_args = cli::CliArgs::parse();

    match cli_args.command {
        cli::Command::Add(args) => add_exercise(args),
        cli::Command::Update(args) => update_exercise(args),
    }
}

fn add_exercise(args: AddArgs) {
    let FullAddArgs {
        slug,
        name,
        difficulty,
    } = args.unwrap_args_or_prompt();

    let config = track_config::PracticeExercise::new(slug.clone(), name, difficulty);

    let mut track_config = TRACK_CONFIG.clone();
    track_config.exercises.practice.push(config);
    let mut new_config = serde_json::to_string_pretty(&track_config)
        .unwrap()
        .to_string();
    new_config += "\n";
    std::fs::write("config.json", new_config).unwrap();

    println!(
        "\
Added your exercise to config.json.
You can add practices, prerequisites and topics if you like."
    );

    make_configlet_generate_what_it_can(&slug);

    let is_update = false;
    generate_exercise_files(&slug, is_update);
}

fn update_exercise(args: UpdateArgs) {
    let slug = args.unwrap_slug_or_prompt();

    make_configlet_generate_what_it_can(&slug);

    let is_update = true;
    generate_exercise_files(&slug, is_update);
}

fn make_configlet_generate_what_it_can(slug: &str) {
    let status = std::process::Command::new("just")
        .args([
            "configlet",
            "sync",
            "--update",
            "--yes",
            "--docs",
            "--metadata",
            "--tests",
            "include",
            "--exercise",
            slug,
        ])
        .status()
        .unwrap();
    if !status.success() {
        panic!("configlet sync failed");
    }
}

fn generate_exercise_files(slug: &str, is_update: bool) {
    let fn_names = if is_update {
        read_fn_names_from_lib_rs(slug)
    } else {
        vec!["TODO".to_string()]
    };

    let exercise = exercise_generation::new(slug, fn_names);

    let exercise_path = PathBuf::from("exercises/practice").join(slug);

    if !is_update {
        std::fs::write(exercise_path.join(".gitignore"), exercise.gitignore).unwrap();
        std::fs::write(exercise_path.join("Cargo.toml"), exercise.manifest).unwrap();
        std::fs::create_dir(exercise_path.join("src")).ok();
        std::fs::write(exercise_path.join("src/lib.rs"), exercise.lib_rs).unwrap();
        std::fs::write(exercise_path.join(".meta/example.rs"), exercise.example).unwrap();
    }

    let template_path = exercise_path.join(".meta/test_template.tera");
    if std::fs::metadata(&template_path).is_err() {
        std::fs::write(template_path, exercise.test_template).unwrap();
    }

    std::fs::create_dir(exercise_path.join("tests")).ok();
    std::fs::write(
        exercise_path.join(format!("tests/{slug}.rs")),
        exercise.tests,
    )
    .unwrap();
}

fn read_fn_names_from_lib_rs(slug: &str) -> Vec<String> {
    let lib_rs =
        std::fs::read_to_string(format!("exercises/practice/{}/src/lib.rs", slug)).unwrap();

    lib_rs
        .split("fn ")
        .skip(1)
        .map(|f| {
            let tmp = f.split_once('(').unwrap().0;
            // strip generics
            if let Some((res, _)) = tmp.split_once('<') {
                res.to_string()
            } else {
                tmp.to_string()
            }
        })
        .collect()
}
