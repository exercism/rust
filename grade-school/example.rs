
use collections::dlist::DList;
use collections::hashmap::HashMap;

pub struct School {
    priv grades: HashMap<uint, DList<~str>>
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(mut self, grade: uint, student: &str) -> School {
        // Keep the list sorted during insert.
        self.grades
            .find_or_insert(grade, DList::new())
            .insert_when(student.into_owned(), |old, new| old > new);
        self
    }

    pub fn sorted(&self) -> ~[(uint, ~[~str])] {
        let mut lst: ~[(uint, ~[~str])] =
            self.grades.iter().map(|(&grade, students)| {
                (grade, students.iter().map(|s| s.clone()).collect())
            }).collect();
        lst.sort();
        lst
    }

    pub fn grade(&self, grade: uint) -> ~[~str] {
        self.grades.find(&grade).map(|students| {
            let ss: ~[~str] = students.iter().map(|s| s.clone()).collect();
            ss
        }).unwrap_or(~[])
    }
}
