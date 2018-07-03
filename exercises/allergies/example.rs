pub struct Allergies(pub usize);

#[derive(PartialEq, Debug)]
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
    pub fn new(score: usize) -> Allergies {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergens = Allergies::allergens();
        let index = allergens
            .iter()
            .position(|x: &Allergen| x == allergen)
            .unwrap();
        (self.0 & (1 << index)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergies::allergens()
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }

    fn allergens() -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}
