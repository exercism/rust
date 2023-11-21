//! This module contains utilities for working with the files in this repo.

pub static REPO_ROOT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

#[test]
fn repo_root_dir_is_correct() {
    let git_database = std::path::PathBuf::from(REPO_ROOT_DIR).join(".git");
    assert!(git_database.is_dir())
}

/// Changes the current working directory to the root of the repository.
///
/// This is intended to be used by executables which operate on files
/// of the repository, so they can use relative paths and still work
/// when called from anywhere within the repository.
pub fn cd_into_repo_root() {
    std::env::set_current_dir(REPO_ROOT_DIR).unwrap();
}
