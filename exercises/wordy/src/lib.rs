pub struct WordProblem;

impl WordProblem {
    pub fn new(command: &str) -> Self {
        unimplemented!(
            "From the '{}' command construct a new WordProblem struct.",
            command
        );
    }

    pub fn answer(&self) -> Option<i32> {
        unimplemented!(
            "Return the result of the transmitted command or None, if the command is invalid."
        );
    }
}
