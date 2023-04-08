pub fn modifier(_ability_score: u8) -> f32 {
    unimplemented!("Implement a utility function that calculates the ability modifier for a given ability score")
}

pub struct Character {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub hitpoints: u8,
}

impl Character {
    pub fn new() -> Self {
        unimplemented!("Create a function that generates a new random character with the following ability scores and hitpoints")
    }

    pub fn roll_four_dice() -> [u8; 4] {
        unimplemented!("Create a utility function that rolls four dice")
    }

    pub fn calculate_ability_score(_ability_dice_rolls: [u8; 4]) -> u8 {
        unimplemented!("Calculate the ability score from the given rolled dice")
    }
}
