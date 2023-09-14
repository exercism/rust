use kindergarten_garden::*;

#[test]
fn garden_with_single_student() {
    let diagram = "RC
GG";
    let student = "Alice";
    let expected = vec!["radishes", "clover", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn different_garden_with_single_student() {
    let diagram = "VC
RC";
    let student = "Alice";
    let expected = vec!["violets", "clover", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn garden_with_two_students() {
    let diagram = "VVCG
VVRC";
    let student = "Bob";
    let expected = vec!["clover", "grass", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn second_students_garden() {
    let diagram = "VVCCGG
VVCCGG";
    let student = "Bob";
    let expected = vec!["clover", "clover", "clover", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn third_students_garden() {
    let diagram = "VVCCGG
VVCCGG";
    let student = "Charlie";
    let expected = vec!["grass", "grass", "grass", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_alice_first_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Alice";
    let expected = vec!["violets", "radishes", "violets", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_bob_second_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Bob";
    let expected = vec!["clover", "grass", "clover", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_charlie() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Charlie";
    let expected = vec!["violets", "violets", "clover", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_david() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "David";
    let expected = vec!["radishes", "violets", "clover", "radishes"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_eve() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Eve";
    let expected = vec!["clover", "grass", "radishes", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_fred() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Fred";
    let expected = vec!["grass", "clover", "violets", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_ginny() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Ginny";
    let expected = vec!["clover", "grass", "grass", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_harriet() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Harriet";
    let expected = vec!["violets", "radishes", "radishes", "violets"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_ileana() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Ileana";
    let expected = vec!["grass", "clover", "violets", "clover"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_joseph() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Joseph";
    let expected = vec!["violets", "clover", "violets", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_kincaid_second_to_last_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Kincaid";
    let expected = vec!["grass", "clover", "clover", "grass"];
    assert_eq!(plants(diagram, student), expected);
}

#[test]
#[ignore]
fn for_larry_last_students_garden() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Larry";
    let expected = vec!["grass", "violets", "clover", "violets"];
    assert_eq!(plants(diagram, student), expected);
}
