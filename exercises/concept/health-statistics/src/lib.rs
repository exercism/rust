// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        unimplemented!()
    }

    pub fn age(&self) -> u32 {
        unimplemented!()
    }

    pub fn weight(&self) -> f32 {
        unimplemented!()
    }

    pub fn age_mut(&mut self, new_age: u32) {
        unimplemented!()
    }

    pub fn weight_mut(&mut self, new_weight: f32) {
        unimplemented!()
    }
}
