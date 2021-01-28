/// This module contains source for the `generate` command.
use crate::{self as exercise, errors::Result, structs::CanonicalData, TEMPLATES};
use failure::format_err;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
use tera::Context;

const GITIGNORE_CONTENT: &str = include_str!("defaults/gitignore");
const EXAMPLE_RS_CONTENT: &str = include_str!("defaults/example.rs");
const DESCRIPTION_MD_CONTENT: &str = include_str!("defaults/description.md");
const METADATA_YML_CONTENT: &str = include_str!("defaults/metadata.yml");

// Generate .meta directory and its contents without using the canonical data
fn generate_meta(exercise_name: &str, exercise_path: &Path) -> Result<()> {
    let meta_dir = exercise_path.join(".meta");
    fs::create_dir(&meta_dir)?;

    for (file, content) in [
        ("description.md", DESCRIPTION_MD_CONTENT),
        ("metadata.yml", METADATA_YML_CONTENT),
    ]
    .iter()
    {
        if !exercise::canonical_file_exists(exercise_name, file)? {
            fs::write(exercise_path.join(".meta").join(file), content)?;
        }
    }

    if fs::read_dir(&meta_dir)?.count() == 0 {
        fs::remove_dir(meta_dir)?;
    }

    Ok(())
}

// Generate test suite using the canonical data
fn generate_tests_from_canonical_data(
    exercise_name: &str,
    tests_path: &Path,
    canonical_data: &CanonicalData,
    use_maplit: bool,
) -> Result<()> {
    exercise::update_cargo_toml_version(exercise_name, canonical_data)?;

    let mut context = Context::from_serialize(canonical_data)?;
    context.insert("use_maplit", &use_maplit);
    context.insert("properties", &canonical_data.properties());

    let test_file = TEMPLATES.render("test_file.rs", &context)?;
    fs::write(&tests_path, test_file)?;

    exercise::rustfmt(&tests_path)?;

    Ok(())
}

// Run bin/configlet generate command to generate README for the exercise
fn generate_readme(exercise_name: &str) -> Result<()> {
    println!(
        "Generating README for {} via 'bin/configlet generate'",
        exercise_name
    );

    let problem_specifications_path = Path::new(&*exercise::TRACK_ROOT)
        .join("..")
        .join("problem-specifications");

    if !problem_specifications_path.exists() {
        let problem_specifications_url = "https://github.com/exercism/problem-specifications.git";
        println!(
            "problem-specifications repository not found. Cloning the repository from {}",
            problem_specifications_url
        );

        Command::new("git")
            .current_dir(&*exercise::TRACK_ROOT)
            .stdout(Stdio::inherit())
            .arg("clone")
            .arg(problem_specifications_url)
            .arg(&problem_specifications_path)
            .output()?;
    }

    exercise::run_configlet_command(
        "generate",
        &[
            ".",
            "--only",
            exercise_name,
            "--spec-path",
            problem_specifications_path
                .to_str()
                .ok_or_else(|| format_err!("path inexpressable as str"))?,
        ],
    )?;

    Ok(())
}

// Generate a new exercise with specified name and flags
pub fn generate_exercise(exercise_name: &str, use_maplit: bool) -> Result<()> {
    if exercise::exercise_exists(exercise_name) {
        return Err(format_err!("exercise with the name {} already exists", exercise_name,).into());
    }

    let exercise_path = Path::new(&*exercise::TRACK_ROOT)
        .join("exercises")
        .join(exercise_name);

    println!(
        "Generating a new exercise at path: {}",
        exercise_path
            .to_str()
            .ok_or_else(|| format_err!("path inexpressable as str"))?
    );

    let _cargo_new_output = Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(
            exercise_path
                .to_str()
                .ok_or_else(|| format_err!("path inexpressable as str"))?,
        )
        .output()?;

    fs::write(exercise_path.join(".gitignore"), GITIGNORE_CONTENT)?;

    if use_maplit {
        let mut cargo_toml_file = OpenOptions::new()
            .append(true)
            .open(exercise_path.join("Cargo.toml"))?;

        cargo_toml_file.write_all(b"maplit = \"1.0.1\"")?;
    }

    fs::create_dir(exercise_path.join("tests"))?;

    let test_file_name = exercise_path
        .join("tests")
        .join(format!("{}.rs", exercise_name));

    fs::write(exercise_path.join("example.rs"), EXAMPLE_RS_CONTENT)?;

    match exercise::get_canonical_data(exercise_name) {
        Ok(canonical_data) => {
            println!("Generating tests from canonical data");

            generate_tests_from_canonical_data(
                &exercise_name,
                &test_file_name,
                &canonical_data,
                use_maplit,
            )?;
        }
        Err(_) => {
            println!(
                "No canonical data for exercise '{}' found. Generating standard exercise template.",
                &exercise_name
            );

            let mut test_file = File::create(test_file_name)?;

            test_file.write_all(
                &format!(
                    "//! Tests for {exercise_name} \n\
                     //! \n\
                     //! Generated by [utility][utility]\n\
                     //! \n\
                     //! [utility]: https://github.com/exercism/rust/tree/main/util/exercise\n\
                     \n",
                    exercise_name = exercise_name,
                )
                .into_bytes(),
            )?;
            if use_maplit {
                test_file.write_all(b"use maplit::hashmap;\n")?;
            }

            test_file.write_all(
                &format!(
                    "use {escaped_exercise_name}::*;\n\n\n",
                    escaped_exercise_name = exercise_name.replace("-", "_"),
                )
                .into_bytes(),
            )?;
        }
    }

    generate_meta(&exercise_name, &exercise_path)?;
    generate_readme(&exercise_name)?;

    Ok(())
}
