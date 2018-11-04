fn main() {
    let branch_name = checker::get_current_branch_name();

    let _modified_exercises: Vec<String> = match branch_name.as_ref() {
        "master" => checker::get_all_exercises(),
        _ => checker::get_modified_exercises(),
    };
}
