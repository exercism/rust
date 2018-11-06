use std::{
    fs,
    path::{Path, PathBuf},
};

fn generate_paths_and_copies(modified_exercise: &Path) -> [(PathBuf, PathBuf); 2] {
    let stub_path = modified_exercise.join("src").join("lib.rs");

    let stub_path_copy = stub_path.with_extension("orig");

    let tests_path = modified_exercise.join("tests");

    let tests_path_copy = tests_path.with_extension("orig");

    [(stub_path, stub_path_copy), (tests_path, tests_path_copy)]
}

fn make_reserve_copies(modified_exercise: &Path) -> Result<(), checker::OSInteractionError> {
    let paths_and_copies = generate_paths_and_copies(modified_exercise);

    let (ref stub_path, ref stub_path_copy) = paths_and_copies[0];

    let (ref tests_path, ref tests_path_copy) = paths_and_copies[1];

    fs::copy(&stub_path, &stub_path_copy).map_err(checker::into_io_command_error!(format!(
        "make a reserve copy for {}",
        &stub_path.display()
    )))?;

    if !tests_path_copy.exists() {
        fs::create_dir(&tests_path_copy).map_err(checker::into_io_command_error!(format!(
            "create a new directory {}",
            &tests_path_copy.display()
        )))?;
    }

    for tests_entry in fs::read_dir(&tests_path).map_err(checker::into_io_command_error!(
        format!("read the {} directory", &tests_path.display())
    ))? {
        if let Ok(tests_entry) = tests_entry {
            let entry_path = tests_entry.path();

            fs::copy(
                &entry_path,
                &tests_path_copy.join(entry_path.file_name().unwrap()),
            ).map_err(checker::into_io_command_error!(format!(
                "make a reserve copy for {}",
                &entry_path.display()
            )))?;
        }
    }

    Ok(())
}

fn remove_copies(modified_exercise: &Path) -> Result<(), checker::OSInteractionError> {
    let paths_and_copies = generate_paths_and_copies(modified_exercise);

    let (ref stub_path, ref stub_path_copy) = paths_and_copies[0];

    let (ref tests_path, ref tests_path_copy) = paths_and_copies[1];

    fs::rename(&stub_path_copy, &stub_path).map_err(checker::into_io_command_error!(format!(
        "rename {} to {}",
        &stub_path_copy.display(),
        &stub_path.display()
    )))?;

    fs::remove_dir_all(&tests_path).map_err(checker::into_io_command_error!(format!(
        "delete the {} directory",
        &tests_path.display()
    )))?;

    fs::rename(&tests_path_copy, &tests_path).map_err(checker::into_io_command_error!(format!(
        "rename {} to {}",
        &tests_path_copy.display(),
        &tests_path.display()
    )))?;

    Ok(())
}

fn add_deny_warning_flag(file_path: &Path) -> Result<(), checker::OSInteractionError> {
    let file_content = fs::read_to_string(&file_path).map_err(checker::into_io_command_error!(
        format!("read the {} file", &file_path.display())
    ))?;

    fs::write(
        &file_path,
        format!("{}\n{}", "#![deny(warnings)]", file_content),
    ).map_err(checker::into_io_command_error!(format!(
        "write to the {} file",
        &file_path.display()
    )))
}

fn make_ignore_warnings(modified_exercise: &Path) -> Result<(), checker::OSInteractionError> {
    let paths_and_copies = generate_paths_and_copies(modified_exercise);

    let (ref stub_path, _) = paths_and_copies[0];

    let (ref tests_path, _) = paths_and_copies[1];

    add_deny_warning_flag(&stub_path)?;

    for entry in fs::read_dir(&tests_path).map_err(checker::into_io_command_error!(format!(
        "read the {} directory",
        &tests_path.display()
    )))? {
        if let Ok(entry) = entry {
            add_deny_warning_flag(&entry.path())?;
        }
    }

    Ok(())
}

fn run_tests(modified_exercise: &Path) -> Result<bool, checker::OSInteractionError> {
    let cargo_test_output = checker::io_command!("cargo test --quiet --no-run");

    let status = cargo_test_output.status;

    if !status.success() {
        println!(
            "COULD NOT COMPILE {}:\n{}",
            modified_exercise.file_name().unwrap().to_str().unwrap(),
            String::from_utf8(cargo_test_output.stderr)?
        );
    }

    Ok(status.success())
}

fn check_stub(modified_exercise: &Path) -> Result<bool, checker::OSInteractionError> {
    let allowed_to_not_compile_flag = modified_exercise
        .join(".meta")
        .join("ALLOWED_TO_NOT_COMPILE");

    if allowed_to_not_compile_flag.exists() {
        println!(
            "The stub for {:?} is allowed to not compile.\n",
            modified_exercise.file_name().unwrap()
        );

        return Ok(true);
    }

    make_reserve_copies(modified_exercise)?;

    make_ignore_warnings(modified_exercise)?;

    let stub_compiled: bool = run_tests(modified_exercise)?;

    remove_copies(modified_exercise)?;

    Ok(stub_compiled)
}

pub fn check_stubs_compile() -> Result<i32, checker::OSInteractionError> {
    let branch_name = checker::get_current_branch_name()?;

    let modified_exercises = match branch_name.as_ref() {
        "master" => checker::get_all_exercises()?,
        _ => checker::get_modified_exercises()?,
    };

    if modified_exercises.is_empty() {
        println!("No exercise was modified. Aborting.");

        return Ok(0);
    } else {
        println!("Found the following modified exercises:");

        modified_exercises.iter().for_each(|modified_exercise| {
            println!(
                "  {}",
                modified_exercise.file_name().unwrap().to_str().unwrap()
            )
        });

        println!();
    }

    let mut broken_exercises = vec![];

    for modified_exercise in &modified_exercises {
        let stub_compiled = check_stub(modified_exercise)?;

        if !stub_compiled {
            broken_exercises.push(modified_exercise.file_name().unwrap().to_str().unwrap());
        }
    }

    if !broken_exercises.is_empty() {
        println!(
            "Some modified stubs could not be compiled. Please make them compile:\n{}",
            broken_exercises
                .iter()
                .map(|broken_exercise| format!(" {}\n", broken_exercise))
                .collect::<String>()
        );

        Ok(1)
    } else {
        println!("All modified stubs compiled successfully.");

        Ok(0)
    }
}
