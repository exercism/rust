use dnd_character::*;

#[test]
fn test_ability_modifier_for_score_3_is_4() {
    let input = 3;
    let expected: f32 = -4.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_4_is_3() {
    let input = 4;
    let expected: f32 = -3.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_5_is_3() {
    let input = 5;
    let expected: f32 = -3.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_6_is_2() {
    let input = 6;
    let expected: f32 = -2.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_7_is_2() {
    let input = 7;
    let expected: f32 = -2.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_8_is_1() {
    let input = 8;
    let expected: f32 = -1.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_9_is_1() {
    let input = 9;
    let expected: f32 = -1.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_10_is_0() {
    let input = 10;
    let expected: f32 = 0.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_11_is_0() {
    let input = 11;
    let expected: f32 = 0.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_12_is_1() {
    let input = 12;
    let expected: f32 = 1.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_13_is_1() {
    let input = 13;
    let expected: f32 = 1.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_14_is_2() {
    let input = 14;
    let expected: f32 = 2.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_15_is_2() {
    let input = 15;
    let expected: f32 = 2.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_16_is_3() {
    let input = 16;
    let expected: f32 = 3.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_17_is_3() {
    let input = 17;
    let expected: f32 = 3.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_ability_modifier_for_score_18_is_4() {
    let input = 18;
    let expected: f32 = 4.0;
    assert_eq!(modifier(input), expected);
}

#[test]
#[ignore]
fn test_random_ability_is_within_range() {
    let score = Character::calculate_ability_score(Character::roll_four_dice());
    assert!((3..=18).contains(&score));
}

#[test]
#[ignore]
fn test_random_character_is_valid() {
    let character = Character::new();
    assert!((3..=18).contains(&character.strength()));
    assert!((3..=18).contains(&character.dexterity()));
    assert!((3..=18).contains(&character.constitution()));
    assert!((3..=18).contains(&character.intelligence()));
    assert!((3..=18).contains(&character.wisdom()));
    assert!((3..=18).contains(&character.charisma()));
    assert_eq!(
        character.hitpoints(),
        10 + modifier(character.constitution()) as u8
    );
}

#[test]
#[ignore]
fn test_dice_are_rolled_for_each_stat() {
    // there is a low chance that this might be equal
    let character = Character::new();
    assert_ne!(
        (
            character.strength(),
            character.dexterity(),
            character.constitution(),
            character.intelligence(),
            character.wisdom(),
            character.charisma()
        ),
        (
            character.strength(),
            character.strength(),
            character.strength(),
            character.strength(),
            character.strength(),
            character.strength()
        )
    );
}

#[test]
#[ignore]
fn test_each_ability_is_only_calculated_once() {
    let character = Character::new();
    assert!(character.strength() == character.strength());
}
