use rand::prelude::*;

pub fn modifier(ability_score: u8) -> f32 {
    ((ability_score as i8 - 10) as f32 / 2.0).floor()
}

pub struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    hitpoints: u8,
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

    pub fn strength(&self) -> u8 {
        self.strength
    }

    pub fn dexterity(&self) -> u8 {
        self.dexterity
    }

    pub fn constitution(&self) -> u8 {
        self.constitution
    }

    pub fn intelligence(&self) -> u8 {
        self.intelligence
    }

    pub fn wisdom(&self) -> u8 {
        self.wisdom
    }

    pub fn charisma(&self) -> u8 {
        self.charisma
    }

    pub fn hitpoints(&self) -> u8 {
        self.hitpoints
    }
}
