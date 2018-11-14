mod missing;
mod outdated;

pub fn list_missing_exercises() {
    missing::list_missing_exercises();
}

pub fn list_outdated_exercises() {
    outdated::list_outdated_exercises();
}
