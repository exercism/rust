pub fn modifier(_ability_score: u8) -> f32 {
    unimplemented!("Implement a utility function that calculates the ability modifier for a given ability score")
}

pub struct Character {}

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

    pub fn strength(&self) -> u8 {
        unimplemented!("Return the strength score of the character")
    }

    pub fn dexterity(&self) -> u8 {
        unimplemented!("Return the dexterity score of the character")
    }

    pub fn constitution(&self) -> u8 {
        unimplemented!("Return the constitution score of the character")
    }

    pub fn intelligence(&self) -> u8 {
        unimplemented!("Return the intelligence score of the character")
    }

    pub fn wisdom(&self) -> u8 {
        unimplemented!("Return the wisdom score of the character")
    }

    pub fn charisma(&self) -> u8 {
        unimplemented!("Return the charisma score of the character")
    }

    pub fn hitpoints(&self) -> u8 {
        unimplemented!("Return the hitpoints of the character")
    }
}
