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
            break "hello-world".to_string();
        } else if !config_content["exercises"]
            .as_array()
            .unwrap()
            .iter()
            .any(|exercise| exercise["slug"] == user_input)
        {
            println!("{} is not an existing exercise slug", user_input);

            continue;
        } else {
            break user_input;
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

fn update_config_content(exercise_name: &str, config_content: &mut Value, user_config: Value) {
    let config_exercises = config_content["exercises"].as_array_mut().unwrap();

    let insert_index = {
        let exercises_with_similar_difficulty = config_exercises
            .iter()
            .enumerate()
            .filter(|(_, exercise)| exercise["difficulty"] == user_config["difficulty"])
            .map(|(index, exercise)| (index, exercise["slug"].as_str().unwrap()))
            .collect::<Vec<(usize, &str)>>();

        let mut start_index = 0;

        let mut end_index = exercises_with_similar_difficulty.len() - 1;

        let insert_index = loop {
            if start_index == end_index {
                break start_index;
            }

            let middle_index = start_index + ((end_index - start_index) / 2);

            let user_input = get_user_input(&format!(
                "Is {} easier then {}? (y/N): ",
                exercise_name, exercises_with_similar_difficulty[middle_index].1
            ));

            if user_input.to_lowercase().starts_with('y') {
                end_index = middle_index;
            } else {
                start_index = middle_index + 1;
            }
        };

        exercises_with_similar_difficulty[insert_index].0
    };

    let prompt = if insert_index == 0 {
        format!("{} is the easiest exercise on the track.", exercise_name)
    } else if insert_index == config_exercises.len() - 1 {
        format!("{} is the hardest exercise on the track.", exercise_name)
    } else {
        format!(
            "{} is placed between {} and {} exercises in difficulty.",
            exercise_name,
            config_exercises[insert_index - 1]["slug"].as_str().unwrap(),
            config_exercises[insert_index]["slug"].as_str().unwrap(),
        )
    };

    println!("You have configured that {}", prompt);

    config_exercises.insert(insert_index, user_config);
}

// TODO: Add a check for the existing exercise configuration
pub fn configure_exercise(exercise_name: &str) {
    println!(
        "Configuring config.json for the {} exercise.",
        exercise_name
    );

    let track_root = utils::get_track_root();

    let config_path = Path::new(&track_root).join("config.json");

    let config_content_string = fs::read_to_string(&config_path)
        .expect("Failed to read the contents of the config.json file");

    let mut config_content: Value = serde_json::from_str(&config_content_string).unwrap();

    let user_config = loop {
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

    update_config_content(exercise_name, &mut config_content, user_config);

    fs::write(
        &config_path,
        serde_json::to_string_pretty(&config_content).unwrap(),
    ).expect("Failed to write the updated track configuration to the config.json file");

    println!("Formatting the config.json file via 'bin/configlet fmt'");

    utils::run_configlate_command("fmt", &["."]);
}
