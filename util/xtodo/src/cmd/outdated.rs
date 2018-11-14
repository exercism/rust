struct OutdatedExercise {
    name: String,
    local_version: Option<String>,
    exercism_version: Option<String>,
}

pub fn list_outdated_exercises() {
    let config = xtodo::get_config_value();

    let _track_name = config.get("language").unwrap().as_str().unwrap();
}
