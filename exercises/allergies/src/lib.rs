pub struct Allergies;

#[derive(Debug, PartialEq)]
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
        unimplemented!(
            "Given the '{}' score, construct a new Allergies struct.",
            score
        );
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        unimplemented!(
            "Determine if the patient is allergic to the '{:?}' allergen.",
            allergen
        );
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
