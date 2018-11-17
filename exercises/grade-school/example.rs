use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.grades.entry(grade).or_insert(Vec::new());
        entry.push(student.to_string());
        entry.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut s = self.grades.keys().cloned().collect::<Vec<u32>>();
        s.sort();
        s
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|v| v.to_vec())
    }
}
