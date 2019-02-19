use failure::Error;
use flate2::read::GzDecoder;
use tar::Archive;

use std::path::{Path, PathBuf};

pub const LATEST_URL: &str = "https://github.com/exercism/configlet/releases/latest";

#[cfg(target_os = "macos")]
pub fn target_os() -> &'static str {
    "mac"
}

#[cfg(target_os = "windows")]
pub fn target_os() -> &'static str {
    "windows"
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn target_os() -> &'static str {
    "linux"
}

#[cfg(target_pointer_width = "32")]
pub fn target_arch() -> &'static str {
    "32bit"
}

#[cfg(not(target_pointer_width = "32"))]
pub fn target_arch() -> &'static str {
    "64bit"
}

pub fn get_latest() -> Result<String, Error> {
    use reqwest::header;

    let response = reqwest::ClientBuilder::new()
        .redirect(reqwest::RedirectPolicy::none())
        .build()?
        .head(LATEST_URL)
        .send()?;

    let location = response
        .headers()
        .get(header::LOCATION)
        .ok_or_else(|| format_err!("location not found in headers\n{:#?}", response.headers()))?;

    Ok(location
        .to_str()?
        .trim()
        .rsplit('/')
        .nth(0)
        .ok_or(format_err!("path separator not in location"))?
        .to_string())
}

/// get the current URL for the appropriate configlet tarball
pub fn pkg_url() -> Result<String, Error> {
    Ok(format!(
        "https://github.com/exercism/configlet/releases/download/{version}/configlet-{os}-{arch}.tgz",
        version=get_latest()?,
        os=target_os(),
        arch=target_arch(),
    ))
}

// download and extract configlet into the repo's /bin folder
//
// returns the path into which the bin was extracted on success
pub fn download() -> Result<PathBuf, Error> {
    let response = reqwest::get(&pkg_url()?)?;
    let mut archive = Archive::new(GzDecoder::new(response));

    let download_path = Path::new(&*crate::TRACK_ROOT).join("bin");
    archive
        .unpack(&download_path)
        .map(|_| download_path)
        .map_err(|e| e.into())
}
