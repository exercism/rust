use tracks_on_tracks_on_tracks::*;

fn check_vec_equality<'a>(a: Vec<&'a str>, b: Vec<&'a str>) {
    assert_eq!(a.len(), b.len());

    for el in &a {
        assert!(b.contains(el));
    }
}

#[test]
pub fn test_list_finished() {
    assert_eq!(list_finished().len(), 0);
}

#[test]
#[ignore]
pub fn test_existing_list() {
    let expected = vec!["Rust", "Clojure", "Elm"];
    check_vec_equality(expected, planned_wishlist());
}

#[test]
#[ignore]
pub fn test_add_language() {
    let expected = vec!["Rust", "Clojure", "Elm", "Bash"];
    let mut languages = vec!["Rust", "Clojure", "Elm"];

    add_to_wishlist(&mut languages, "Bash");

    check_vec_equality(expected, languages);
}

#[test]
#[ignore]
pub fn test_count_languages() {
    let expected = vec!["Rust", "Clojure", "Elm"];
    assert_eq!(3, count_languages(expected));
}

#[test]
#[ignore]
pub fn has_language_yes() {
    let expected = vec!["Rust", "Clojure", "Elm"];
    assert!(has_language(expected, "Elm"));
}

#[test]
#[ignore]
pub fn has_language_no() {
    let expected = vec!["Rust", "Clojure", "Elm"];
    assert!(!has_language(expected, "D"));
}

#[test]
#[ignore]
pub fn test_reverse_list() {
    let expected = vec!["Elm", "Clojure", "Rust"];
    let languages = vec!["Rust", "Clojure", "Elm"];

    check_vec_equality(expected, reverse_wishlist(languages));
}

#[test]
#[ignore]
pub fn is_exciting_yes() {
    let expected = vec!["Rust", "Clojure", "Elm"];
    assert!(!is_exciting(expected));
}

#[test]
#[ignore]
pub fn is_exciting_too_many() {
    let languages = vec!["VBA", "Rust", "Clojure", "Elm"];
    assert!(!is_exciting(languages));
}

#[test]
#[ignore]
pub fn is_exciting_empty() {
    let languages: Vec<&str> = Vec::new();
    assert!(!is_exciting(languages));
}

#[test]
#[ignore]
pub fn is_exciting_single_star() {
    let languages: Vec<&str> = vec!["Rust"];
    assert!(is_exciting(languages));
}

#[test]
#[ignore]
pub fn is_exciting_star_on_second_place_size2() {
    let languages: Vec<&str> = vec!["F#", "Rust"];
    assert!(is_exciting(languages));
}

#[test]
#[ignore]
pub fn is_exciting_star_on_second_place_size3() {
    let languages: Vec<&str> = vec!["F#", "Rust", "Clojure"];
    assert!(is_exciting(languages));
}

#[test]
#[ignore]
pub fn remove_language_yes() {
    let expected = vec!["Rust", "Elm"];
    let mut languages = vec!["Rust", "Clojure", "Elm"];

    remove_from_wishlist(&mut languages, "Clojure");
    check_vec_equality(expected, languages);
}

#[test]
#[ignore]
pub fn remove_language_no() {
    let expected = vec!["Rust", "Clojure", "Elm"];
    let mut languages = vec!["Rust", "Clojure", "Elm"];

    remove_from_wishlist(&mut languages, "English");
    check_vec_equality(expected, languages);
}

#[test]
#[ignore]
pub fn is_unique_yes() {
    let languages = vec!["Rust", "Clojure", "Elm"];
    assert!(is_unique(languages));
}

#[test]
#[ignore]
pub fn is_unique_no() {
    let mut languages = vec!["Rust", "Clojure", "Elm"];
    languages.push("Rust");
    assert!(!is_unique(languages));
}
