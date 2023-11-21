use models::exercise_config::get_all_exercise_paths;

/// The package manifest of each exercise should not contain an `authors` field.
/// The authors are already specified in the track configuration.
#[test]
fn no_authors_in_cargo_toml() {
    let cargo_toml_paths = get_all_exercise_paths().map(|p| format!("{p}/Cargo.toml"));

    for path in cargo_toml_paths {
        let cargo_toml = std::fs::read_to_string(path).unwrap();
        assert!(
            !cargo_toml.contains("authors"),
            "Cargo.toml should not contain an 'authors' field"
        );
    }
}
