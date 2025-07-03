#[derive(Clone, PartialEq, Eq)]
pub struct Tree {}

impl Tree {
    #[must_use]
    pub fn new(_label: &str) -> Self {
        todo!("Implement a function that creates a new Tree with given label");
    }

    #[must_use]
    pub fn with_children(_label: &str, _children: &[Tree]) -> Self {
        todo!("Implement a function that creates a new Tree with given label and Children");
    }

    #[must_use]
    pub fn get_label(&self) -> String {
        todo!("Implement getter for label.");
    }

    #[must_use]
    pub fn get_children(&self) -> Vec<&Self> {
        todo!("Implement getter for children.");
    }

    #[must_use]
    pub fn pov_from(self, _from: &str) -> Option<Self> {
        todo!("Implement a function that reparents Tree with 'from' as root.");
    }

    #[must_use]
    pub fn path_to(self, _from: &str, _to: &str) -> Option<Vec<String>>
    {
        todo!("Implement a function that returns the list of labels in the shortest path from 'from' to 'to'");
    }
}
