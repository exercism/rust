pub struct WordProblem;

impl WordProblem {
    pub fn new(command: &str) -> Self {
        unimplemented!(
            "From the '{}' command construct a new WordProblem struct.",
            command
        );
    }

    pub fn answer(&self) -> Result<i32, String> {
        unimplemented!(
            "Return the result of the transmitted command, or an error, if the command is invalid."
        );
    }
}
