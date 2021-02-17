pub struct CsvRecordBuilder {
    content: String,
}

impl CsvRecordBuilder {
    // Create a new builder
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Adds an item to the list separated by a space and a comma.
    pub fn add(&mut self, val: &str) {
        if !self.content.is_empty() {
            self.content.push(',');
        }

        if val.contains(",") || val.contains(r#"""#) || val.contains("\n") {
            self.content.push('"');
            self.content.push_str(&val.replace(r#"""#, r#""""#));
            self.content.push('"');
        } else {
            self.content.push_str(val);
        }
    }

    /// Consumes the builder and returns the CSV record.
    pub fn build(self) -> String {
        self.content
    }
}
