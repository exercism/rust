use failure::Error;
use flate2::read::GzDecoder;
use git2::Repository;
use tar::Archive;

use std::path::PathBuf;

pub const LATEST_URL: &'static str = "https://github.com/exercism/configlet/releases/latest";

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
        .get::<header::Location>()
        .ok_or_else(|| format_err!("location not found in headers\n{}", response.headers()))?;
    Ok(location
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

/// return the path to the bin directory of the repo
pub fn binpath() -> Result<PathBuf, Error> {
    let repo = Repository::discover(".")?;
    repo.workdir()
        .ok_or(format_err!("repo has no working directory"))
        .map(|p| {
            let mut b = p.to_path_buf();
            b.push("bin");
            b
        })
}

// download and extract configlet into the repo's /bin folder
//
// returns the path into which the bin was extracted on success
pub fn download() -> Result<PathBuf, Error> {
    let path = binpath()?;
    let response = reqwest::get(&pkg_url()?)?;
    let mut archive = Archive::new(GzDecoder::new(response));
    archive.unpack(&path).map(|_| path).map_err(|e| e.into())
}
