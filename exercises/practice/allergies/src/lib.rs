pub struct Allergies;

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        todo!("Given the '{score}' score, construct a new Allergies struct.");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        todo!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
