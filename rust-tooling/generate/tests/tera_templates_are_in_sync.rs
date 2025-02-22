//! This test is not run in CI. Doing so would slow down CI noticeably, because
//! exercise generation depends on `tera`, which takes a while to compile.

#![allow(clippy::unwrap_used, clippy::panic)]

use glob::glob;
use utils::fs::cd_into_repo_root;

#[test]
fn tera_templates_are_in_sync() {
    cd_into_repo_root();
    for entry in glob("exercises/*/*/.meta/test_template.tera").unwrap() {
        let path = entry.unwrap();
        let exercise_dir = path.parent().unwrap().parent().unwrap();
        let slug = exercise_dir.file_name().unwrap().to_string_lossy();

        let generated = generate::new(&slug);

        let snake_slug = slug.replace('-', "_");
        let test_path = exercise_dir.join("tests").join(format!("{snake_slug}.rs"));
        let on_disk = std::fs::read_to_string(test_path).unwrap();

        if generated.tests != on_disk {
            let diff = difference::Changeset::new(&on_disk, &generated.tests, "\n");
            println!("DIFF: ===\n{diff}");
            println!("GENERATED (sanity): ===\n{}", generated.tests);

            panic!(
                "
    The Tera template for exercise '{slug}' is not in sync.
    Run 'just update-exercise --slug {slug}' to fix it.\n"
            )
        }
    }
}
