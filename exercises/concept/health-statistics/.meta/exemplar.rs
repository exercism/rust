pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User { name, age, weight }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn age_mut(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn weight_mut(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}
