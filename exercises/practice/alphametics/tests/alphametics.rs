use std::collections::HashMap;

fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
    let answer = alphametics::solve(puzzle);
    let solution: HashMap<char, u8> = solution.iter().cloned().collect();
    assert_eq!(answer, Some(solution));
}

#[test]
fn test_with_three_letters() {
    assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
}

#[test]
#[ignore]
fn test_must_have_unique_value_for_each_letter() {
    let answer = alphametics::solve("A == B");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn test_leading_zero_solution_is_invalid() {
    let answer = alphametics::solve("ACA + DD == BD");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn test_sum_must_be_wide_enough() {
    let answer = alphametics::solve("ABC + DEF == GH");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn puzzle_with_two_digits_final_carry() {
    assert_alphametic_solution_eq(
        "A + A + A + A + A + A + A + A + A + A + A + B == BCC",
        &[('A', 9), ('B', 1), ('C', 0)],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_four_letters() {
    assert_alphametic_solution_eq("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
}

#[test]
#[ignore]
fn test_puzzle_with_six_letters() {
    assert_alphametic_solution_eq(
        "NO + NO + TOO == LATE",
        &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_seven_letters() {
    assert_alphametic_solution_eq(
        "HE + SEES + THE == LIGHT",
        &[
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_eight_letters() {
    assert_alphametic_solution_eq(
        "SEND + MORE == MONEY",
        &[
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_ten_letters() {
    assert_alphametic_solution_eq(
        "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
        &[
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
        ],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_ten_letters_and_199_addends() {
    assert_alphametic_solution_eq(
        "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        &[
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
        ],
    );
}
