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
        if self
            .grades
            .iter()
            .any(|(_, students)| students.iter().any(|s| s == student))
        {
            return; // don't add duplicate student
        }
        let entry = self.grades.entry(grade).or_default();
        entry.push(student.to_string());
        entry.sort_unstable();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut s = self.grades.keys().cloned().collect::<Vec<u32>>();
        s.sort_unstable();
        s
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .map(|v| v.to_vec())
            .unwrap_or_default()
    }
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}
