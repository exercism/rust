use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    name: String,

    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
        attrs.iter().for_each(|&(name, value)| {
            self.attrs.insert(name.to_string(), value.to_string());
        });

        self
    }
}
