use std::{
    error, fmt, io,
    path::{Path, PathBuf},
    process::Command,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum OSInteractionError {
    IOCommandError(String, io::Error),
    ParseOutputError(FromUtf8Error),
}

impl fmt::Display for OSInteractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OSInteractionError::IOCommandError(ref failed_command, ref io_error) => write!(
                f,
                "Failed to run the '{}' command:\n{}\n",
                failed_command, io_error
            ),
            OSInteractionError::ParseOutputError(ref parse_error) => {
                write!(f, "Failed to parse command output:\n{}\n", parse_error)
            }
        }
    }
}

impl error::Error for OSInteractionError {
    fn description(&self) -> &str {
        match *self {
            OSInteractionError::IOCommandError(_, ref io_error) => io_error.description(),
            OSInteractionError::ParseOutputError(ref parse_error) => parse_error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            OSInteractionError::IOCommandError(_, ref io_error) => Some(io_error),
            OSInteractionError::ParseOutputError(ref parse_error) => Some(parse_error),
        }
    }
}

impl From<FromUtf8Error> for OSInteractionError {
    fn from(err: FromUtf8Error) -> OSInteractionError {
        OSInteractionError::ParseOutputError(err)
    }
}

macro_rules! into_io_command_error {
    ($failed_command: expr) => {
        |io_error| OSInteractionError::IOCommandError($failed_command, io_error)
    };
}

fn get_track_root() -> Result<String, OSInteractionError> {
    let rev_parse_output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
        .map_err(into_io_command_error!(
            "git rev-parse --show-toplevel".to_string()
        ))?;

    Ok(String::from_utf8(rev_parse_output.stdout)?
        .trim()
        .to_string())
}

pub fn get_all_exercises() -> Result<Vec<PathBuf>, OSInteractionError> {
    let track_root = get_track_root()?;

    let exercises_path = Path::new(&track_root).join("exercises");

    let exercises_dir = exercises_path
        .read_dir()
        .map_err(into_io_command_error!(format!(
            "read {} directory",
            &exercises_path.display()
        )))?;

    Ok(exercises_dir
        .filter(|entry| entry.is_ok() && entry.as_ref().unwrap().path().is_dir())
        .map(|entry| entry.unwrap().path().to_path_buf())
        .collect())
}

pub fn get_current_branch_name() -> Result<String, OSInteractionError> {
    let rev_parse_output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(into_io_command_error!(
            "git rev-parse --abbrev-ref HEAD".to_string()
        ))?;

    Ok(String::from_utf8(rev_parse_output.stdout)?
        .trim()
        .to_string())
}

fn get_modifications() -> Result<Vec<String>, OSInteractionError> {
    let diff_output = Command::new("git")
        .args(&["diff", "--name-only", "master"])
        .output()
        .map_err(into_io_command_error!(
            "git diff --name-only master".to_string()
        ))?;

    Ok(String::from_utf8(diff_output.stdout)?
        .trim()
        .split('\n')
        .map(|x| x.to_owned())
        .collect())
}

pub fn get_modified_exercises() -> Result<Vec<PathBuf>, OSInteractionError> {
    let modifications = get_modifications()?;

    let track_root = get_track_root()?;

    Ok(modifications
        .iter()
        .filter(|modification_path| modification_path.contains("exercises"))
        .map(|modification_path| {
            Path::new(modification_path)
                .iter()
                .take(2)
                .collect::<PathBuf>()
        }).map(|modified_exercise| Path::new(&track_root).join(modified_exercise).to_path_buf())
        .collect())
}
