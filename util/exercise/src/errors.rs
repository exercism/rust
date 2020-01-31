use failure;
use reqwest;
use serde_json;
use std::{convert::From, io, result};
use tera;
use toml;

#[derive(Debug, failure::Fail)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    IoError(#[cause] io::Error),
    #[fail(
        display = "config.json malformed: '{}' must have field '{}'",
        parent, field
    )]
    ConfigJsonSchemaError { parent: String, field: String },
    #[fail(
        display = "{} malformed: field '{}' must have type '{}'",
        file, field, as_type
    )]
    SchemaTypeError {
        file: String,
        field: String,
        as_type: String,
    },
    #[fail(display = "json error: {}", _0)]
    JsonError(#[cause] serde_json::Error),
    #[fail(display = "config.toml parse error: {}", _0)]
    ConfigTomlParseError(#[cause] toml::de::Error),
    #[fail(display = "HTTP error: {}", _0)]
    HTTPError(reqwest::Error),
    #[fail(display = "Tera rendering error: {}", _0)]
    TeraError(tera::Error),
    #[fail(display = "{}", _0)]
    Failure(#[cause] failure::Error),
    #[fail(display = "Unknown Failure: {}", _0)]
    UnknownError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ConfigTomlParseError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HTTPError(err)
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

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Error::TeraError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
