use exercise::{self, Result};
use serde_json::{self, Value};
use std::{
    fs,
    io::{stdin, stdout, Write},
    path::Path,
};
use uuid::Uuid;

fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);

    let mut buffer = String::new();

    stdout().flush()?;
    stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

fn get_user_config(exercise_name: &str, config_content: &Value) -> Result<Value> {
    let existing_config = get!(config_content, "exercises", as_array)
        .iter()
        .find(|exercise| exercise["slug"] == exercise_name);

    let uuid = if let Some(existing_config) = existing_config {
        get!(existing_config, "uuid", as_str).to_string()
    } else {
        Uuid::new_v4().to_hyphenated().to_string()
    };

    let unlocked_by: Option<String> = loop {
        let default_value = if let Some(existing_config) = existing_config {
            if let Value::String(s) = get!(existing_config, "unlocked_by") {
                s
            } else {
                "none"
            }
        } else {
            "hello-world"
        };

        let user_input = get_user_input(&format!(
            "Exercise slug which unlocks this (blank for '{}'): ",
            default_value
        ))?;

        if user_input.is_empty() {
            break Some(default_value.to_string());
        } else if user_input == "null" {
            break None;
        } else if !get!(config_content, "exercises", as_array)
            .iter()
            .any(|exercise| exercise["slug"] == user_input)
        {
            println!("{} is not an existing exercise slug", user_input);
            continue;
        } else {
            break Some(user_input);
        };
    };

    let core = unlocked_by.is_none();

    let difficulty = loop {
        let unlocked_by_difficulty = match unlocked_by {
            Some(ref unlocked_by) => {
                let unlocked_by_exercise = get!(config_content, "exercises", as_array)
                    .iter()
                    .find(|exercise| exercise["slug"] == unlocked_by.as_str())
                    .ok_or(format_err!(
                        "exercise '{}' not found in config",
                        unlocked_by
                    ))?;

                get!(unlocked_by_exercise, "difficulty", as_u64)
            }

            None => 1,
        };

        let available_difficulties: Vec<u64> = [1, 4, 7, 10]
            .iter()
            .skip_while(|&&difficulty| difficulty < unlocked_by_difficulty)
            .cloned()
            .collect();

        let default_value = if let Some(existing_config) = existing_config {
            get!(existing_config, "difficulty", as_u64)
        } else {
            *available_difficulties
                .first()
                .ok_or(format_err!("no available difficulties"))?
        };

        let user_input = get_user_input(&format!(
            "Difficulty for this exercise {:?} (blank for {}): ",
            available_difficulties, default_value
        ))?;

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
            use serde_json::Value::*;
            use std::string;
            match get!(existing_config, "topics") {
                String(s) => vec![s.to_string()],
                Array(topics_values) => {
                    let mut topics: Vec<string::String> = Vec::with_capacity(topics_values.len());
                    for topic in topics_values.iter() {
                        topics.push(val_as!(topic, as_str).to_string())
                    }
                    topics
                }
                _ => {
                    return Err(exercise::errors::Error::SchemaTypeError {
                        file: "config.json".to_string(),
                        field: "topics".to_string(),
                        as_type: "array or string".to_string(),
                    })
                }
            }
        } else {
            vec![exercise_name.to_string()]
        };

        let user_input = get_user_input(&format!(
            "List of topics for this exercise, comma-separated (blank for {:?}): ",
            default_value,
        ))?;

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

    Ok(json!({
        "slug": exercise_name,
        "uuid": uuid,
        "core": core,
        "unlocked_by": unlocked_by,
        "difficulty": difficulty,
        "topics": topics
    }))
}

fn choose_exercise_insert_index(
    exercise_name: &str,
    exercises: &[Value],
    difficulty: &Value,
) -> Result<usize> {
    loop {
        let mut exercises_with_similar_difficulty = Vec::with_capacity(exercises.len());
        for (index, exercise) in exercises
            .iter()
            .enumerate()
            .filter(|(_, exercise)| exercise["difficulty"] == *difficulty)
        {
            exercises_with_similar_difficulty.push((index, get!(exercise, "slug", as_str)));
        }

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
            ))?;

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
                get!(exercises[insert_index - 1], "slug", as_str),
                get!(exercises[insert_index], "slug", as_str),
            )
        };

        let user_input = get_user_input(&format!(
            "You have configured that {}.\nIs this correct? (y/N): ",
            prompt
        ))?;

        if user_input.to_lowercase().starts_with('y') {
            break Ok(insert_index);
        }
    }
}

fn insert_user_config(
    exercise_name: &str,
    config_content: &mut Value,
    user_config: Value,
) -> Result<()> {
    let exercises = get_mut!(config_content, "exercises", as_array_mut);

    let insert_index =
        choose_exercise_insert_index(exercise_name, exercises, &user_config["difficulty"])?;

    exercises.insert(insert_index, user_config);

    Ok(())
}

fn update_existing_config(
    exercise_name: &str,
    config_content: &mut Value,
    user_config: Value,
) -> Result<()> {
    let exercises = get_mut!(config_content, "exercises", as_array_mut);

    let existing_exercise_index = exercises
        .iter()
        .position(|exercise| exercise["slug"] == exercise_name)
        .ok_or(format_err!(
            "exercise '{}' not found in config.json",
            exercise_name
        ))?;

    let insert_index =
        if exercises[existing_exercise_index]["difficulty"] == user_config["difficulty"] {
            existing_exercise_index
        } else {
            choose_exercise_insert_index(exercise_name, &exercises, &user_config["difficulty"])?
        };

    exercises.remove(existing_exercise_index);

    exercises.insert(insert_index, user_config);

    Ok(())
}

pub fn configure_exercise(exercise_name: &str) -> Result<()> {
    println!(
        "Configuring config.json for the {} exercise.",
        exercise_name
    );

    let config_path = Path::new(&*exercise::TRACK_ROOT).join("config.json");

    let config_content_string = fs::read_to_string(&config_path)?;

    let mut config_content: Value = serde_json::from_str(&config_content_string)?;

    let config_exists = get!(config_content, "exercises", as_array)
        .iter()
        .any(|exercise| exercise["slug"] == exercise_name);

    let user_config: Value = loop {
        let user_config = get_user_config(exercise_name, &config_content)?;

        let user_input = get_user_input(&format!(
            "You have configured the {} exercise as follows:\n{}\nIs this correct? (y/N):",
            exercise_name,
            serde_json::to_string_pretty(&user_config)?
        ))?;

        if user_input.to_lowercase().starts_with('y') {
            break user_config;
        }
    };

    if config_exists {
        update_existing_config(exercise_name, &mut config_content, user_config)?;
    } else {
        insert_user_config(exercise_name, &mut config_content, user_config)?;
    }

    fs::write(&config_path, serde_json::to_string_pretty(&config_content)?)?;

    println!("Formatting the config.json file via 'bin/configlet fmt'");

    exercise::run_configlet_command("fmt", &["."])?;

    Ok(())
}
