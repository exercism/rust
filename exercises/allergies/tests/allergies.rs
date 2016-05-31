extern crate allergies;

use allergies::*;

#[test]
fn test_no_allergies_means_not_allergic() {
    let allergies = Allergies::new(0);
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Peanuts));
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Cats));
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
#[ignore]
fn test_is_allergic_to_eggs() {
    assert_eq!(true, Allergies::new(1).is_allergic_to(&Allergen::Eggs));
}

#[test]
#[ignore]
fn test_has_the_right_allergies() {
    let allergies = Allergies::new(5);
    assert_eq!(true, allergies.is_allergic_to(&Allergen::Eggs));
    assert_eq!(true, allergies.is_allergic_to(&Allergen::Shellfish));
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
#[ignore]
fn test_no_allergies_at_all() {
    assert_eq!(Vec::<Allergen>::new(), Allergies::new(0).allergies());
}

#[test]
#[ignore]
fn test_just_to_peanuts() {
    assert_eq!(vec![Allergen::Peanuts], Allergies::new(2).allergies());
}

#[test]
#[ignore]
fn test_allergic_to_everything() {
    let all = vec![Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish,
                   Allergen::Strawberries, Allergen::Tomatoes,
                   Allergen::Chocolate, Allergen::Pollen, Allergen::Cats];
    let allergies = Allergies::new(255).allergies();

    assert_eq!(allergies.len(), all.len());

    for allergy in &all {
        assert!(allergies.contains(&allergy));
    }
}

#[test]
#[ignore]
fn test_ignore_non_allergen_score_parts() {
    assert_eq!(vec![Allergen::Eggs], Allergies::new(257).allergies());
}
