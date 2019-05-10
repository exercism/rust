use crate::errors::Result;
use failure::format_err;
use flate2::read::GzDecoder;
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    string::ToString,
};
use tar::Archive;

// Returns the "os-arch" string, according to the host machine parameters
fn get_os_arch() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "mac"
    } else {
        "linux"
    };
    let arch = if cfg!(target_pointer_width = "32") {
        "32bit"
    } else {
        "64bit"
    };
    format!("{}-{}", os, arch)
}

// Makes a request to the Github API to get and return the download url for the latest configlet release
fn get_download_url() -> Result<String> {
    reqwest::get("https://api.github.com/repos/exercism/configlet/releases/latest")?
        .json::<Value>()?
        .get("assets")
        .and_then(Value::as_array)
        .ok_or_else(|| format_err!("failed to get the 'assets' field from the Github response"))?
        .iter()
        .filter_map(|asset| asset.get("browser_download_url").and_then(Value::as_str))
        .find(|url| url.contains(&get_os_arch()))
        .map(ToString::to_string)
        .ok_or_else(|| format_err!("failed to get the configlet release url"))
        .map_err(|e| e.into())
}

// download and extract configlet into the repo's /bin folder
//
// returns the path into which the bin was extracted on success
pub fn download() -> Result<PathBuf> {
    let response = reqwest::get(&get_download_url()?)?;
    let mut archive = Archive::new(GzDecoder::new(response));
    let download_path = Path::new(&*crate::TRACK_ROOT).join("bin");
    archive
        .unpack(&download_path)
        .map(|_| download_path)
        .map_err(|e| e.into())
}
