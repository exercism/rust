use allergies::*;

fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
    for element in expected {
        if !actual.contains(element) {
            panic!("Allergen missing\n  {element:?} should be in {actual:?}");
        }
    }

    if actual.len() != expected.len() {
        panic!(
            "Allergy vectors are of different lengths\n  expected {expected:?}\n  got {actual:?}"
        );
    }
}

#[test]

fn not_allergic_to_anything_eggs() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Eggs))
}

#[test]
#[ignore]

fn allergic_only_to_eggs_eggs() {
    let allergies = Allergies::new(1);
    assert!(allergies.is_allergic_to(&Allergen::Eggs))
}

#[test]
#[ignore]

fn allergic_to_eggs_and_something_else_eggs() {
    let allergies = Allergies::new(3);
    assert!(allergies.is_allergic_to(&Allergen::Eggs))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_eggs_eggs() {
    let allergies = Allergies::new(2);
    assert!(!allergies.is_allergic_to(&Allergen::Eggs))
}

#[test]
#[ignore]

fn allergic_to_everything_eggs() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Eggs))
}

#[test]
#[ignore]

fn not_allergic_to_anything_peanuts() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Peanuts))
}

#[test]
#[ignore]

fn allergic_only_to_peanuts_peanuts() {
    let allergies = Allergies::new(2);
    assert!(allergies.is_allergic_to(&Allergen::Peanuts))
}

#[test]
#[ignore]

fn allergic_to_peanuts_and_something_else_peanuts() {
    let allergies = Allergies::new(7);
    assert!(allergies.is_allergic_to(&Allergen::Peanuts))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_peanuts_peanuts() {
    let allergies = Allergies::new(5);
    assert!(!allergies.is_allergic_to(&Allergen::Peanuts))
}

#[test]
#[ignore]

fn allergic_to_everything_peanuts() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Peanuts))
}

#[test]
#[ignore]

fn not_allergic_to_anything_shellfish() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Shellfish))
}

#[test]
#[ignore]

fn allergic_only_to_shellfish_shellfish() {
    let allergies = Allergies::new(4);
    assert!(allergies.is_allergic_to(&Allergen::Shellfish))
}

#[test]
#[ignore]

fn allergic_to_shellfish_and_something_else_shellfish() {
    let allergies = Allergies::new(14);
    assert!(allergies.is_allergic_to(&Allergen::Shellfish))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_shellfish_shellfish() {
    let allergies = Allergies::new(10);
    assert!(!allergies.is_allergic_to(&Allergen::Shellfish))
}

#[test]
#[ignore]

fn allergic_to_everything_shellfish() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Shellfish))
}

#[test]
#[ignore]

fn not_allergic_to_anything_strawberries() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries))
}

#[test]
#[ignore]

fn allergic_only_to_strawberries_strawberries() {
    let allergies = Allergies::new(8);
    assert!(allergies.is_allergic_to(&Allergen::Strawberries))
}

#[test]
#[ignore]

fn allergic_to_strawberries_and_something_else_strawberries() {
    let allergies = Allergies::new(28);
    assert!(allergies.is_allergic_to(&Allergen::Strawberries))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_strawberries_strawberries() {
    let allergies = Allergies::new(20);
    assert!(!allergies.is_allergic_to(&Allergen::Strawberries))
}

#[test]
#[ignore]

fn allergic_to_everything_strawberries() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Strawberries))
}

#[test]
#[ignore]

fn not_allergic_to_anything_tomatoes() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Tomatoes))
}

#[test]
#[ignore]

fn allergic_only_to_tomatoes_tomatoes() {
    let allergies = Allergies::new(16);
    assert!(allergies.is_allergic_to(&Allergen::Tomatoes))
}

#[test]
#[ignore]

fn allergic_to_tomatoes_and_something_else_tomatoes() {
    let allergies = Allergies::new(56);
    assert!(allergies.is_allergic_to(&Allergen::Tomatoes))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_tomatoes_tomatoes() {
    let allergies = Allergies::new(40);
    assert!(!allergies.is_allergic_to(&Allergen::Tomatoes))
}

#[test]
#[ignore]

fn allergic_to_everything_tomatoes() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Tomatoes))
}

#[test]
#[ignore]

fn not_allergic_to_anything_chocolate() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Chocolate))
}

#[test]
#[ignore]

fn allergic_only_to_chocolate_chocolate() {
    let allergies = Allergies::new(32);
    assert!(allergies.is_allergic_to(&Allergen::Chocolate))
}

#[test]
#[ignore]

fn allergic_to_chocolate_and_something_else_chocolate() {
    let allergies = Allergies::new(112);
    assert!(allergies.is_allergic_to(&Allergen::Chocolate))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_chocolate_chocolate() {
    let allergies = Allergies::new(80);
    assert!(!allergies.is_allergic_to(&Allergen::Chocolate))
}

#[test]
#[ignore]

fn allergic_to_everything_chocolate() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Chocolate))
}

#[test]
#[ignore]

fn not_allergic_to_anything_pollen() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Pollen))
}

#[test]
#[ignore]

fn allergic_only_to_pollen_pollen() {
    let allergies = Allergies::new(64);
    assert!(allergies.is_allergic_to(&Allergen::Pollen))
}

#[test]
#[ignore]

fn allergic_to_pollen_and_something_else_pollen() {
    let allergies = Allergies::new(224);
    assert!(allergies.is_allergic_to(&Allergen::Pollen))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_pollen_pollen() {
    let allergies = Allergies::new(160);
    assert!(!allergies.is_allergic_to(&Allergen::Pollen))
}

#[test]
#[ignore]

fn allergic_to_everything_pollen() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Pollen))
}

#[test]
#[ignore]

fn not_allergic_to_anything_cats() {
    let allergies = Allergies::new(0);
    assert!(!allergies.is_allergic_to(&Allergen::Cats))
}

#[test]
#[ignore]

fn allergic_only_to_cats_cats() {
    let allergies = Allergies::new(128);
    assert!(allergies.is_allergic_to(&Allergen::Cats))
}

#[test]
#[ignore]

fn allergic_to_cats_and_something_else_cats() {
    let allergies = Allergies::new(192);
    assert!(allergies.is_allergic_to(&Allergen::Cats))
}

#[test]
#[ignore]

fn allergic_to_something_but_not_cats_cats() {
    let allergies = Allergies::new(64);
    assert!(!allergies.is_allergic_to(&Allergen::Cats))
}

#[test]
#[ignore]

fn allergic_to_everything_cats() {
    let allergies = Allergies::new(255);
    assert!(allergies.is_allergic_to(&Allergen::Cats))
}

#[test]
#[ignore]

fn no_allergies() {
    let allergies = Allergies::new(0).allergies();
    let expected = &[];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn just_eggs() {
    let allergies = Allergies::new(1).allergies();
    let expected = &[Allergen::Eggs];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn just_peanuts() {
    let allergies = Allergies::new(2).allergies();
    let expected = &[Allergen::Peanuts];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn just_strawberries() {
    let allergies = Allergies::new(8).allergies();
    let expected = &[Allergen::Strawberries];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn eggs_and_peanuts() {
    let allergies = Allergies::new(3).allergies();
    let expected = &[Allergen::Eggs, Allergen::Peanuts];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn more_than_eggs_but_not_peanuts() {
    let allergies = Allergies::new(5).allergies();
    let expected = &[Allergen::Eggs, Allergen::Shellfish];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn lots_of_stuff() {
    let allergies = Allergies::new(248).allergies();
    let expected = &[
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn everything() {
    let allergies = Allergies::new(255).allergies();
    let expected = &[
        Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn no_allergen_score_parts() {
    let allergies = Allergies::new(509).allergies();
    let expected = &[
        Allergen::Eggs,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats,
    ];

    compare_allergy_vectors(expected, &allergies);
}

#[test]
#[ignore]

fn no_allergen_score_parts_without_highest_valid_score() {
    let allergies = Allergies::new(257).allergies();
    let expected = &[Allergen::Eggs];

    compare_allergy_vectors(expected, &allergies);
}
