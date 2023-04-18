pub fn list_finished() -> Vec<&'static str> {
    vec![]
}

pub fn planned_wishlist() -> Vec<&'static str> {
    vec!["Rust", "Clojure", "Elm"]
}

pub fn add_to_wishlist<'a>(languages: &mut Vec<&'a str>, lang: &'a str) {
    languages.push(lang);
}

pub fn count_languages(languages: Vec<&str>) -> usize {
    languages.len()
}

pub fn has_language(languages: Vec<&str>, lang: &str) -> bool {
    for _lang in languages {
        if lang == _lang {
            return true;
        }
    }

    false
}

pub fn reverse_wishlist(languages: Vec<&str>) -> Vec<&str> {
    let mut result: Vec<&str> = vec![];

    let mut i = (languages.len() - 1) as i32;

    while i >= 0 {
        result.push(languages[i as usize]);
        i -= 1;
    }

    result
}

pub fn is_exciting(languages: Vec<&str>) -> bool {
    let len = languages.len();

    if len == 0 {
        return false;
    }

    if len == 2 || len == 3 {
        return languages[1] == "Rust";
    }

    languages[0] == "Rust"
}

pub fn remove_from_wishlist(languages: &mut Vec<&str>, lang: &str) {
    let mut index = 0;

    let len = languages.len();

    while index < len {
        if languages[index] == lang {
            languages.remove(index);
            break;
        }
        index += 1;
    }
}

pub fn is_unique(mut languages: Vec<&str>) -> bool {
    let original_length = languages.len();
    languages.sort();
    languages.dedup();
    let deduped_lenghh = languages.len();

    original_length == deduped_lenghh
}
