use std::fs;

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

fn get_file_lines(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect(&format!("Could not read {}", file_name))
        .split("\n")
        .map(|line| line.to_string())
        .collect()
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Vec<String> {
    let mut grep_result = vec![];

    let is_multiple_file_search = files.len() > 1;

    for file_name in files {
        let file_lines = get_file_lines(file_name);

        grep_result.extend(
            file_lines
                .iter()
                .enumerate()
                .filter(|&(_, line)| {
                    let mut inner_line = String::from(line.clone());

                    let mut inner_pattern = String::from(pattern);

                    if flags.use_caseinsensitive_comparison {
                        inner_line = inner_line.to_lowercase().to_string();

                        inner_pattern = inner_pattern.to_lowercase().to_string();
                    }

                    let mut is_filtered = inner_line.contains(&inner_pattern);

                    if flags.match_entire_line {
                        is_filtered = inner_line == inner_pattern;
                    }

                    if flags.use_inverted_comparison {
                        is_filtered = !inner_line.contains(&inner_pattern);
                    }

                    is_filtered
                })
                .filter(|(_, line)| !line.is_empty())
                .map(|(line_number, line)| {
                    let mut result = line.to_owned();

                    if flags.print_line_number {
                        result.insert_str(0, &format!("{}:", line_number + 1));
                    }

                    if is_multiple_file_search {
                        result.insert_str(0, &format!("{}:", file_name))
                    }

                    if flags.print_file_name {
                        result = file_name.to_owned().to_owned();
                    }

                    result
                }),
        );
    }

    grep_result.dedup_by(|a, b| (*a).eq(b));

    grep_result
}
