use std::path::PathBuf;

fn main() {
    let branch_name = checker::get_current_branch_name();

    let modified_exercises: Vec<PathBuf> = match branch_name.as_ref() {
        "master" => checker::get_all_exercises(),
        _ => checker::get_modified_exercises(),
    };

    if modified_exercises.is_empty() {
        println!("No exercise was modified. Aborting.");

        return;
    } else {
        println!(
            "Found the following modified exercises:\n{:#?}",
            modified_exercises
        )
    }
}
