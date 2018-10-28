use std::path::PathBuf;
use std::process::Command;

// we look for the track root in various places, but it's never going to change
// we therefore cache the value for efficiency
lazy_static! {
    pub static ref TRACK_ROOT: String = {
        let rev_parse_output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .expect("Failed to get the path to the track repo.");

        String::from_utf8(rev_parse_output.stdout)
            .expect("git rev-parse produced non-utf8 output")
            .trim()
            .to_string()
    };
}

#[inline]
pub fn track_root() -> PathBuf {
    PathBuf::from(&*TRACK_ROOT)
}

#[inline]
pub fn bin() -> PathBuf {
    track_root().join("bin")
}

#[inline]
#[cfg(target_os = "windows")]
pub fn configlet() -> PathBuf {
    bin().join("configlet.exe")
}

#[inline]
#[cfg(not(target_os = "windows"))]
pub fn configlet() -> PathBuf {
    bin().join("configlet")
}

#[inline]
pub fn config_json() -> PathBuf {
    track_root().join("config.json")
}

#[inline]
pub fn exercise(name: &str) -> PathBuf {
    track_root().join("exercises").join(name)
}

#[inline]
pub fn cargo_toml(name: &str) -> PathBuf {
    exercise(name).join("Cargo.toml")
}

#[inline]
pub fn dot_meta(name: &str) -> PathBuf {
    exercise(name).join(".meta")
}

#[inline]
pub fn tests_dir(name: &str) -> PathBuf {
    exercise(name).join("tests")
}

pub fn tests(name: &str) -> PathBuf {
    tests_dir(name).join(format!("{}.rs", name))
}
