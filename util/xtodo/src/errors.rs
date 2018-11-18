use std::{io, string};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    IOError(#[cause] io::Error),

    #[fail(display = "UTF8 parsing error: {}", _0)]
    UTF8Error(#[cause] string::FromUtf8Error),

    #[fail(display = "json error: {}", _0)]
    JsonError(#[cause] serde_json::Error),

    #[fail(display = "HTTP error: {}", _0)]
    HTTPError(reqwest::Error),

    #[fail(display = "config.toml parse error: {}", _0)]
    ConfigTomlParseError(#[cause] toml::de::Error),

    #[fail(display = "{}", _0)]
    Failure(#[cause] failure::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::UTF8Error(err)
    }
}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Self {
        Error::Failure(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HTTPError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ConfigTomlParseError(err)
    }
}
