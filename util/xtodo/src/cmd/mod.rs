mod missing;
mod outdated;

pub fn list_missing_exercises() -> xtodo::Result<()> {
    missing::list_missing_exercises()
}

pub fn list_outdated_exercises() -> xtodo::Result<()> {
    outdated::list_outdated_exercises()
}
