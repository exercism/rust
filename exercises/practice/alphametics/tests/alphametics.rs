use alphametics::*;

#[test]
fn puzzle_with_three_letters() {
    let answer = solve("I + BB == ILL");
    let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn solution_must_have_unique_value_for_each_letter() {
    let answer = solve("A == B");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn leading_zero_solution_is_invalid() {
    let answer = solve("ACA + DD == BD");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn puzzle_with_two_digits_final_carry() {
    let answer = solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC");
    let expected = [('A', 9), ('B', 1), ('C', 0)].into_iter().collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn puzzle_with_four_letters() {
    let answer = solve("AS + A == MOM");
    let expected = [('A', 9), ('S', 2), ('M', 1), ('O', 0)]
        .into_iter()
        .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn puzzle_with_six_letters() {
    let answer = solve("NO + NO + TOO == LATE");
    let expected = [('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]
        .into_iter()
        .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn puzzle_with_seven_letters() {
    let answer = solve("HE + SEES + THE == LIGHT");
    let expected = [
        ('E', 4),
        ('G', 2),
        ('H', 5),
        ('I', 0),
        ('L', 1),
        ('S', 9),
        ('T', 7),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn puzzle_with_eight_letters() {
    let answer = solve("SEND + MORE == MONEY");
    let expected = [
        ('S', 9),
        ('E', 5),
        ('N', 6),
        ('D', 7),
        ('M', 1),
        ('O', 0),
        ('R', 8),
        ('Y', 2),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn puzzle_with_ten_letters() {
    let answer = solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");
    let expected = [
        ('A', 5),
        ('D', 3),
        ('E', 4),
        ('F', 7),
        ('G', 8),
        ('N', 0),
        ('O', 2),
        ('R', 1),
        ('S', 6),
        ('T', 9),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}

#[test]
#[ignore]
fn puzzle_with_ten_letters_and_199_addends() {
    let answer = solve("THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES");
    let expected = [
        ('A', 1),
        ('E', 0),
        ('F', 5),
        ('H', 8),
        ('I', 7),
        ('L', 2),
        ('O', 6),
        ('R', 3),
        ('S', 4),
        ('T', 9),
    ]
    .into_iter()
    .collect();
    assert_eq!(answer, Some(expected));
}
