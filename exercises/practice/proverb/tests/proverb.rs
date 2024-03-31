use proverb::*;

#[test]
fn zero_pieces() {
    let input = &[];
    let output = build_proverb(input);
    let expected = String::new();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_piece() {
    let input = &["nail"];
    let output = build_proverb(input);
    let expected: String = ["And all for the want of a nail."].join("\n");
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_pieces() {
    let input = &["nail", "shoe"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_pieces() {
    let input = &["nail", "shoe", "horse"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn full_proverb() {
    let input = &[
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "For want of a horse the rider was lost.",
        "For want of a rider the message was lost.",
        "For want of a message the battle was lost.",
        "For want of a battle the kingdom was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn four_pieces_modernized() {
    let input = &["pin", "gun", "soldier", "battle"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a pin the gun was lost.",
        "For want of a gun the soldier was lost.",
        "For want of a soldier the battle was lost.",
        "And all for the want of a pin.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
