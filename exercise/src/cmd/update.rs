use std::path::Path;
use utils;

fn exercise_exists(exercise_name: &str) -> bool {
    let track_root = utils::get_track_root();

    let exercises_path = Path::new(&track_root).join("exercises");

    for entry in exercises_path
        .read_dir()
        .expect("Failed to read 'exercises' dir")
    {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_dir() && entry.file_name() == exercise_name {
                return true;
            }
        }
    }

    false
}

pub fn update_exercise(exercise_name: &str) {
    if !exercise_exists(exercise_name) {
        panic!(
            "Exercise with the name '{}' does not exists. Aborting",
            exercise_name
        );
    }
}
