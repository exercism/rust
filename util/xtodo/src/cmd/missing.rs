pub fn list_missing_exercises() {
    let all_exercises = reqwest::get(
        "https://api.github.com/repos/exercism/problem-specifications/contents/exercises/",
    ).unwrap()
    .text()
    .unwrap();

    println!("{}", all_exercises);
}
