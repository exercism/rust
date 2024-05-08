use anyhow::Error;

use std::{fs, path::Path};

#[derive(Debug, thiserror::Error)]
enum FileAccessError {
    #[error("File not found: {}", file_name)]
    FileNotFoundError { file_name: String },

    #[error("Error reading file: {}", file_name)]
    FileReadError { file_name: String },
}

pub struct Flags {
    print_line_number: bool,
    print_file_name: bool,
    use_caseinsensitive_comparison: bool,
    use_inverted_comparison: bool,
    match_entire_line: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            print_line_number: flags.contains(&"-n"),
            print_file_name: flags.contains(&"-l"),
            use_caseinsensitive_comparison: flags.contains(&"-i"),
            use_inverted_comparison: flags.contains(&"-v"),
            match_entire_line: flags.contains(&"-x"),
        }
    }
}

fn get_file_lines(file_name: &str) -> Result<Vec<String>, FileAccessError> {
    let file_path = Path::new(file_name);

    if !file_path.exists() {
        return Err(FileAccessError::FileNotFoundError {
            file_name: String::from(file_name),
        });
    }

    if let Ok(content) = fs::read_to_string(file_path) {
        Ok(content.lines().map(|line| line.to_string()).collect())
    } else {
        Err(FileAccessError::FileReadError {
            file_name: String::from(file_name),
        })
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut grep_result = vec![];

    let is_multiple_file_search = files.len() > 1;

    for file_name in files {
        let file_lines = get_file_lines(file_name)?;

        grep_result.extend(
            file_lines
                .iter()
                .enumerate()
                .filter(|&(_, line)| {
                    let mut inner_line = line.clone();

                    let mut inner_pattern = String::from(pattern);

                    if flags.use_caseinsensitive_comparison {
                        inner_line = inner_line.to_lowercase();

                        inner_pattern = inner_pattern.to_lowercase();
                    }

                    if flags.use_inverted_comparison {
                        !inner_line.contains(&inner_pattern)
                    } else if flags.match_entire_line {
                        inner_line == inner_pattern
                    } else {
                        inner_line.contains(&inner_pattern)
                    }
                })
                .map(|(line_number, line)| {
                    let mut result = line.to_owned();

                    if flags.print_line_number {
                        result.insert_str(0, &format!("{}:", line_number + 1));
                    }

                    if is_multiple_file_search {
                        result.insert_str(0, &format!("{}:", file_name))
                    }

                    if flags.print_file_name {
                        file_name.to_owned().clone_into(&mut result);
                    }

                    result
                }),
        );
    }

    grep_result.dedup_by(|a, b| (*a).eq(b));

    Ok(grep_result)
}
