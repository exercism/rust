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
    let existing_config: Option<&Value> = config_content["exercises"]
        .as_array()
        .unwrap()
        .iter()
        .find(|exercise| exercise["slug"] == exercise_name);

    let uuid = if let Some(existing_config) = existing_config {
        existing_config["uuid"].as_str().unwrap().to_string()
    } else {
        Uuid::new_v4().to_hyphenated().to_string()
    };

    let core = false;

    let unlocked_by = loop {
        let default_value = if let Some(existing_config) = existing_config {
            existing_config["unlocked_by"].as_str().unwrap()
        } else {
            "hello-world"
        };

        let user_input = get_user_input(&format!(
            "Exercise slug which unlocks this (blank for '{}'): ",
            default_value
        ));

        if user_input.is_empty() {
            break default_value.to_string();
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
        let unlocked_by_difficulty = config_content["exercises"]
            .as_array()
            .unwrap()
            .iter()
            .find(|exercise| exercise["slug"] == unlocked_by)
            .unwrap()["difficulty"]
            .as_u64()
            .unwrap();

        let available_difficulties: Vec<u64> = [1, 4, 7, 10]
            .iter()
            .skip_while(|&&difficulty| difficulty < unlocked_by_difficulty)
            .cloned()
            .collect();

        let default_value = if let Some(existing_config) = existing_config {
            existing_config["difficulty"].as_u64().unwrap()
        } else {
            *available_difficulties.first().unwrap()
        };

        let user_input = get_user_input(&format!(
            "Difficulty for this exercise {:?} (blank for {}): ",
            available_difficulties, default_value
        ));

        if user_input.is_empty() {
            break default_value;
        } else if let Ok(difficulty) = user_input.parse::<u64>() {
            if !available_difficulties.contains(&difficulty) {
                println!(
                    "Difficulty should be {:?}, not '{}'.",
                    available_difficulties, difficulty
                );

                continue;
            }

            break difficulty;
        } else {
            println!("Difficulty should be a number, not '{}'.", user_input);

            continue;
        }
    };

    let topics = loop {
        let default_value = if let Some(existing_config) = existing_config {
            let topics = &existing_config["topics"];

            if topics.is_array() {
                topics
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|topic| topic.as_str().unwrap().to_string())
                    .collect::<Vec<String>>()
            } else {
                vec![topics.as_str().unwrap().to_string()]
            }
        } else {
            vec![exercise_name.to_string()]
        };

        let user_input = get_user_input(&format!(
            "List of topics for this exercise, comma-separated (blank for {:?}): ",
            default_value,
        ));

        if user_input.is_empty() {
            break default_value;
        }

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

fn choose_exercise_insert_index(
    exercise_name: &str,
    exercises: &[Value],
    difficulty: &Value,
) -> usize {
    loop {
        let exercises_with_similar_difficulty = exercises
            .iter()
            .enumerate()
            .filter(|(_, exercise)| exercise["difficulty"] == *difficulty)
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

        let insert_index = exercises_with_similar_difficulty[insert_index].0;

        let prompt = if insert_index == 0 {
            format!(
                "{} is the easiest exercise of difficulty {}.",
                exercise_name, *difficulty
            )
        } else if insert_index == exercises.len() - 1 {
            format!(
                "{} is the hardest exercise of difficulty {}.",
                exercise_name, *difficulty
            )
        } else {
            format!(
                "{} is placed between {} and {} exercises in difficulty.",
                exercise_name,
                exercises[insert_index - 1]["slug"].as_str().unwrap(),
                exercises[insert_index]["slug"].as_str().unwrap(),
            )
        };

        let user_input = get_user_input(&format!(
            "You have configured that {}.\nIs this correct? (y/N): ",
            prompt
        ));

        if user_input.to_lowercase().starts_with('y') {
            break insert_index;
        }
    }
}

fn insert_user_config(exercise_name: &str, config_content: &mut Value, user_config: Value) {
    let exercises = config_content["exercises"].as_array_mut().unwrap();

    let insert_index =
        choose_exercise_insert_index(exercise_name, exercises, &user_config["difficulty"]);

    exercises.insert(insert_index, user_config);
}

fn update_existing_config(exercise_name: &str, config_content: &mut Value, user_config: Value) {
    let exercises = config_content["exercises"].as_array_mut().unwrap();

    let existing_exercise_index = exercises
        .iter()
        .position(|exercise| exercise["slug"] == exercise_name)
        .unwrap();

    let insert_index =
        if exercises[existing_exercise_index]["difficulty"] == user_config["difficulty"] {
            existing_exercise_index
        } else {
            choose_exercise_insert_index(exercise_name, &exercises, &user_config["difficulty"])
        };

    exercises.remove(existing_exercise_index);

    exercises.insert(insert_index, user_config);
}

pub fn configure_exercise(exercise_name: &str) {
    println!(
        "Configuring config.json for the {} exercise.",
        exercise_name
    );

    let config_path = Path::new(&*utils::TRACK_ROOT).join("config.json");

    let config_content_string = fs::read_to_string(&config_path)
        .expect("Failed to read the contents of the config.json file");

    let mut config_content: Value = serde_json::from_str(&config_content_string).unwrap();

    let config_exists = config_content["exercises"]
        .as_array()
        .unwrap()
        .iter()
        .any(|exercise| exercise["slug"] == exercise_name);

    let user_config: Value = loop {
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

    if config_exists {
        update_existing_config(exercise_name, &mut config_content, user_config);
    } else {
        insert_user_config(exercise_name, &mut config_content, user_config);
    }

    fs::write(
        &config_path,
        serde_json::to_string_pretty(&config_content).unwrap(),
    )
    .expect("Failed to write the updated track configuration to the config.json file");

    println!("Formatting the config.json file via 'bin/configlet fmt'");

    utils::run_configlet_command("fmt", &["."]);
}
