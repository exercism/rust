use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.grades.entry(grade) {
            Entry::Vacant(view) => {
                view.insert(vec![student.to_string()]);
            }
            Entry::Occupied(mut view) => {
                let l = view.get_mut();
                l.push(student.to_string());
                l.sort();
            }
        };
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut s = self.grades.keys().map(|k| k.clone()).collect::<Vec<u32>>();
        s.sort();
        s
    }

    pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
        self.grades.get(&grade)
    }
}
