use std::{fs, path::Path, process::Command};

fn make_reserve_copies(modified_exercise: &Path) {
    let stub_path = modified_exercise.join("src").join("lib.rs");

    let stub_path_copy = modified_exercise.join("src").join("lib.rs.orig");

    let tests_path = modified_exercise.join("tests");

    let tests_path_copy = modified_exercise.join("tests.orig");

    fs::copy(&stub_path, &stub_path_copy)
        .unwrap_or_else(|_| panic!("Failed to make a reserve copy for {:?}", &stub_path));

    if !tests_path_copy.exists() {
        fs::create_dir(&tests_path_copy)
            .unwrap_or_else(|_| panic!("Failed to create {:?} directory", &tests_path_copy));
    }

    for tests_entry in fs::read_dir(&tests_path)
        .unwrap_or_else(|_| panic!("Failed to read {:?} directory", &tests_path))
    {
        if let Ok(tests_entry) = tests_entry {
            let entry_path = tests_entry.path();
            fs::copy(
                &entry_path,
                &tests_path_copy.join(entry_path.file_name().unwrap()),
            ).unwrap_or_else(|_| panic!("Failed to make a reserve copy for {:?}", entry_path));
        }
    }
}

fn remove_copies(modified_exercise: &Path) {
    let stub_path = modified_exercise.join("src").join("lib.rs");

    let stub_path_copy = modified_exercise.join("src").join("lib.rs.orig");

    let tests_path = modified_exercise.join("tests");

    let tests_path_copy = modified_exercise.join("tests.orig");

    fs::rename(&stub_path_copy, &stub_path).unwrap_or_else(|error| {
        panic!(
            "Failed to rename {:?} to {:?}: {}",
            &stub_path_copy, &stub_path, error
        )
    });

    fs::remove_dir_all(&tests_path)
        .unwrap_or_else(|error| panic!("Failed to delete directory {:?}: {}", &tests_path, error));

    fs::rename(&tests_path_copy, &tests_path).unwrap_or_else(|error| {
        panic!(
            "Failed to rename {:?} to {:?}: {}",
            &tests_path_copy, &tests_path, error
        )
    });
}

fn add_deny_warning_flag(file_path: &Path) {
    let file_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|error| panic!("Failed to read file {:?}: {}", &file_path, error));

    fs::write(
        &file_path,
        format!("{}\n{}", "#![deny(warnings)]", file_content),
    ).unwrap_or_else(|error| panic!("Failed to write to file {:?}: {}", &file_path, error));
}

fn make_ignore_warnings(modified_exercise: &Path) {
    let stub_path = modified_exercise.join("src").join("lib.rs");

    let tests_path = modified_exercise.join("tests");

    add_deny_warning_flag(&stub_path);

    for entry in fs::read_dir(&tests_path)
        .unwrap_or_else(|error| panic!("Failed to read directory {:?}: {}", &tests_path, error))
    {
        if let Ok(entry) = entry {
            add_deny_warning_flag(&entry.path());
        }
    }
}

fn run_tests(modified_exercise: &Path) -> bool {
    let cargo_test_output = Command::new("cargo")
        .args(&["test", "--quiet", "--no-run"])
        .current_dir(&modified_exercise)
        .output()
        .expect("Failed to run cargo test command");

    let status = cargo_test_output.status;

    if !status.success() {
        println!(
            "COULD NOT COMPILE {}:\n{}",
            modified_exercise.file_name().unwrap().to_str().unwrap(),
            String::from_utf8_lossy(&cargo_test_output.stderr)
        );
    }

    status.success()
}

fn check_stub(modified_exercise: &Path) -> bool {
    let allowed_to_not_compile_flag = modified_exercise
        .join(".meta")
        .join("ALLOWED_TO_NOT_COMPILE");

    if allowed_to_not_compile_flag.exists() {
        println!(
            "The stub for {:?} is allowed to not compile.\n",
            modified_exercise.file_name().unwrap()
        );

        return true;
    }

    make_reserve_copies(modified_exercise);

    make_ignore_warnings(modified_exercise);

    let stub_compiled: bool = run_tests(modified_exercise);

    remove_copies(modified_exercise);

    stub_compiled
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

    if modified_exercises
        .iter()
        .map(|modified_exercise| check_stub(modified_exercise))
        .any(|stub_compiled| !stub_compiled)
    {
        println!("Some modified stubs could not be compiled.\nPlease make them compile.");

        Ok(1)
    } else {
        println!("All modified stubs compiled successfully.");

        Ok(0)
    }
}
