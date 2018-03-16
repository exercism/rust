use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount,
    InvalidColumnCount,
}

pub fn convert(input: &str) -> Result<String, Error> {
    if input.lines().count() % 4 != 0 {
        return Err(Error::InvalidRowCount);
    }
    if input.lines().any(|line| line.chars().count() % 3 != 0) {
        return Err(Error::InvalidColumnCount);
    }

    let y = input.lines().collect::<Vec<_>>();

    let mut converted_lines = vec![];

    for char_lines in y.chunks(4) {
        let mut unparsed_characters = BTreeMap::new();

        for line in char_lines {
            let line_chars = line.chars().collect::<Vec<_>>();

            for (char_number, char_chunk) in line_chars.chunks(3).enumerate() {
                let char_chars = unparsed_characters.entry(char_number).or_insert(vec![]);
                for c in char_chunk {
                    char_chars.push(*c);
                }
            }
        }

        let mut parsed_characters = String::new();

        for (_, v) in unparsed_characters {
            parsed_characters.push(convert_character(&v));
        }

        converted_lines.push(parsed_characters.to_string());
    }

    Ok(converted_lines.join(","))
}

fn convert_character(input: &Vec<char>) -> char {
    if &input[..] == [' ', '_', ' ', '|', ' ', '|', '|', '_', '|', ' ', ' ', ' '] {
        '0'
    } else if &input[..] == [' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '1'
    } else if &input[..] == [' ', '_', ' ', ' ', '_', '|', '|', '_', ' ', ' ', ' ', ' '] {
        '2'
    } else if &input[..] == [' ', '_', ' ', ' ', '_', '|', ' ', '_', '|', ' ', ' ', ' '] {
        '3'
    } else if &input[..] == [' ', ' ', ' ', '|', '_', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '4'
    } else if &input[..] == [' ', '_', ' ', '|', '_', ' ', ' ', '_', '|', ' ', ' ', ' '] {
        '5'
    } else if &input[..] == [' ', '_', ' ', '|', '_', ' ', '|', '_', '|', ' ', ' ', ' '] {
        '6'
    } else if &input[..] == [' ', '_', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '7'
    } else if &input[..] == [' ', '_', ' ', '|', '_', '|', '|', '_', '|', ' ', ' ', ' '] {
        '8'
    } else if &input[..] == [' ', '_', ' ', '|', '_', '|', ' ', '_', '|', ' ', ' ', ' '] {
        '9'
    } else {
        '?'
    }
}
