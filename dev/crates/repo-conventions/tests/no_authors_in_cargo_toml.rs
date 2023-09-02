use track_config::TRACK_CONFIG;

/// The package manifest of each exercise should not contain an `authors` field.
/// The authors are already specified in the track configuration.
#[test]
fn test_no_authors_in_cargo_toml() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    let cargo_toml_paths = TRACK_CONFIG
        .exercises
        .concept
        .iter()
        .map(|e| format!("concept/{}", e.slug))
        .chain(
            TRACK_CONFIG
                .exercises
                .practice
                .iter()
                .map(|e| format!("practice/{}", e.slug)),
        )
        .map(|s| format!("{crate_dir}/../../../exercises/{s}/Cargo.toml"));

    for path in cargo_toml_paths {
        let cargo_toml = std::fs::read_to_string(path).unwrap();
        assert!(
            !cargo_toml.contains("authors"),
            "Cargo.toml should not contain an 'authors' field"
        );
    }
}
