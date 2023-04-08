use rand::prelude::*;

pub fn modifier(ability_score: u8) -> f32 {
    ((ability_score as i8 - 10) as f32 / 2.0).floor()
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
        let constitution = Character::calculate_ability_score(Character::roll_four_dice());
        Character {
            strength: Character::calculate_ability_score(Character::roll_four_dice()),
            dexterity: Character::calculate_ability_score(Character::roll_four_dice()),
            constitution,
            intelligence: Character::calculate_ability_score(Character::roll_four_dice()),
            wisdom: Character::calculate_ability_score(Character::roll_four_dice()),
            charisma: Character::calculate_ability_score(Character::roll_four_dice()),
            hitpoints: 10 + modifier(constitution) as u8,
        }
    }

    pub fn roll_four_dice() -> [u8; 4] {
        let mut rng = thread_rng();
        let mut rolls = [0; 4];
        for i in 0..4 {
            rolls[i] = rng.gen_range(1..=4)
        }
        rolls
    }

    pub fn calculate_ability_score(ability_dice_rolls: [u8; 4]) -> u8 {
        let mut ability_dice_rolls = ability_dice_rolls;
        ability_dice_rolls.sort();
        ability_dice_rolls[1..].iter().sum()
    }
}
