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
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn age(&self) -> u32 {
        todo!()
    }

    pub fn weight(&self) -> f32 {
        todo!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        todo!()
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        todo!()
    }
}
