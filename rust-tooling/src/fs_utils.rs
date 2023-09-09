//! This module contains utilities for working with the files in this repo.

/// Changes the current working directory to the root of the repository.
///
/// This is intended to be used by executables which operate on files
/// of the repository, so they can use relative paths and still work
/// when called from anywhere within the repository.
pub fn cd_into_repo_root() {
    static RUST_TOOLING_DIR: &str = env!("CARGO_MANIFEST_DIR");
    let repo_root_dir = std::path::PathBuf::from(RUST_TOOLING_DIR).join("..");
    std::env::set_current_dir(repo_root_dir).unwrap();
}
