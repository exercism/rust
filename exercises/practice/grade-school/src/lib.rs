use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    roster: BTreeMap<String, u32>
}

impl School {
    pub fn new() -> School {
        School{
            ..Default::default()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(student.to_string()).or_insert(grade); 
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res:Vec<u32> = self.roster.values().cloned().collect::<BTreeSet<_>>().into_iter().collect();
        res.sort();
        res
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster.iter().filter_map(|(n, &g)| {
            if g == grade {
                Some(n.clone())
            }
            else {
                None
            }
        }).collect()
    }
}
