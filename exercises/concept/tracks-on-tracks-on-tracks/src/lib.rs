#![allow(clippy::ptr_arg)]
pub fn list_finished() -> Vec<&'static str> {
    unimplemented!("Implement list_finished")
}

pub fn planned_wishlist() -> Vec<&'static str> {
    unimplemented!("Implement planned_wishlist")
}

pub fn add_to_wishlist<'a>(_languages: &mut Vec<&'a str>, _lang: &'a str) {
    unimplemented!("Implement add_to_wishlist")
}

pub fn count_languages(_languages: Vec<&str>) -> usize {
    unimplemented!("Implement count_languages")
}

pub fn has_language(_languages: Vec<&str>, _lang: &str) -> bool {
    unimplemented!("Implement has_language")
}

pub fn reverse_wishlist(_languages: Vec<&str>) -> Vec<&str> {
    unimplemented!("Implement reverse_wishlist")
}

pub fn is_exciting(_languages: Vec<&str>) -> bool {
    unimplemented!("Implement is_exciting")
}

pub fn remove_from_wishlist(_languages: &mut Vec<&str>, _lang: &str) {
    unimplemented!("Implement remove_from_wishlist")
}

pub fn is_unique(mut _languages: Vec<&str>) -> bool {
    unimplemented!("Implement is_unique")
}
