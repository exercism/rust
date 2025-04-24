use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    score: u32
}

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 0b00000001,
    Peanuts = 0b00000010,
    Shellfish = 0b00000100,
    Strawberries = 0b00001000,
    Tomatoes = 0b00010000,
    Chocolate = 0b00100000,
    Pollen = 0b01000000,
    Cats = 0b10000000,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
        .filter(|a| self.is_allergic_to(a))
        .collect()
    }
}
