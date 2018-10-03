use serde_json::{self, Value};
use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
};
use utils;
use uuid::Uuid;

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);

    let mut buffer = String::new();

    stdout().flush().unwrap();

    stdin()
        .read_line(&mut buffer)
        .expect("Failed to get user input.");

    buffer.trim().to_string()
}

fn get_user_config(exercise_name: &str, config_content: &Value) -> Value {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let core = false;

    let unlocked_by = loop {
        let user_input = get_user_input("Exercise slug which unlocks this (blank for None): ");
        if user_input.is_empty() {
            // TODO: As of Exercism V2, only core exercises can have "unlocked_by"
            // field set to null. Perhaps "hello-world" could be used here?
            break None;
        } else if !config_content["exercises"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value["slug"] == user_input)
        {
            println!("{} is not an existing exercise slug", user_input);

            continue;
        } else {
            break Some(user_input);
        };
    };

    let difficulty = loop {
        let user_input = get_user_input("Difficulty for this exercise(1, 4, 7, 10): ");

        if let Ok(difficulty) = user_input.parse::<u32>() {
            if ![1, 4, 7, 10].contains(&difficulty) {
                println!("Difficulty should be 1, 4, 7 or 10, not '{}'.", difficulty);

                continue;
            }

            break difficulty;
        } else {
            println!("Difficulty should be a number, not '{}'.", user_input);

            continue;
        }
    };

    let topics = loop {
        let user_input = get_user_input("List of topics for this exercise, comma-separated: ");

        let topics = user_input
            .split(',')
            .map(|topic| topic.trim().to_string())
            .filter(|topic| !topic.is_empty())
            .collect::<Vec<String>>();

        if topics.is_empty() {
            println!("Must enter at least one topic");

            continue;
        }

        break topics;
    };

    json!({
        "slug": exercise_name,
        "uuid": uuid,
        "core": core,
        "unlocked_by": unlocked_by,
        "difficulty": difficulty,
        "topics": topics
    })
}

pub fn configure_exercise(exercise_name: &str) {
    println!(
        "Configuring config.json for the {} exercise.",
        exercise_name
    );

    let track_root = utils::get_track_root();

    let config_path = Path::new(&track_root).join("config.json");

    let config_content_string = fs::read_to_string(config_path)
        .expect("Failed to read the contents of the config.json file");

    let config_content: Value = serde_json::from_str(&config_content_string).unwrap();

    let _user_config = loop {
        let user_config = get_user_config(exercise_name, &config_content);

        let user_input = get_user_input(&format!(
            "You have configured the {} exercise as follows:\n{}\nIs this correct? (y/N):",
            exercise_name,
            serde_json::to_string_pretty(&user_config).unwrap()
        ));

        if user_input.to_lowercase().starts_with('y') {
            break user_config;
        }
    };
}
