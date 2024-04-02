//! This test is not run in CI. Doing so would slow down CI noticeably, because
//! exercise generation depends on `tera`, which takes a while to compile.

use glob::glob;
use utils::fs::cd_into_repo_root;

#[test]
fn tera_templates_are_in_sync() {
    cd_into_repo_root();
    for entry in glob("exercises/*/*/.meta/test_template.tera").unwrap() {
        let path = entry.unwrap();
        let exercise_dir = path.parent().unwrap().parent().unwrap();
        let slug = exercise_dir.file_name().unwrap().to_string_lossy();

        let fn_names = generate::read_fn_names_from_lib_rs(&slug);
        let generated = generate::new(&slug, fn_names);

        let test_path = exercise_dir.join("tests").join(format!("{slug}.rs"));
        let on_disk = std::fs::read_to_string(test_path).unwrap();

        if generated.tests != on_disk {
            panic!(
                "
    The Tera template for exercise '{slug}' is not in sync.
    Run 'just update-exercise --slug {slug}' to fix it.\n"
            )
        }
    }
}
