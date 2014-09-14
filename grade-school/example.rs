use std::collections::TreeMap;

pub struct School {
    grades: TreeMap<uint, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School { grades: TreeMap::new() }
    }

    pub fn add(&mut self, grade: uint, student: &str) {
        let mut insert = false;
        match self.grades.find_mut(&grade) {
            None => { insert = true },
            Some(l) => { l.push(student.into_string()); l.sort() },
        };
        if insert {
            self.grades.insert(grade, vec!(student.into_string()));
            ()
        }
    }
    
    pub fn grades(&self) -> Vec<uint> {
        let mut s = self.grades.keys().map(|k| k.clone()).collect::<Vec<uint>>();
        s.sort();
        s
    }

    pub fn grade(&self, grade: uint) -> Option<&Vec<String>> {
        self.grades.find(&grade)
    }
}
