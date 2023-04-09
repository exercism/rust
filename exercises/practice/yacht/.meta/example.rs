pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => get_score(dice, 1),
        Category::Twos => get_score(dice, 2),
        Category::Threes => get_score(dice, 3),
        Category::Fours => get_score(dice, 4),
        Category::Fives => get_score(dice, 5),
        Category::Sixes => get_score(dice, 6),
        Category::FullHouse => get_full_house(dice),
        Category::FourOfAKind => get_poker(dice),
        Category::LittleStraight => get_straight(dice, 'l'),
        Category::BigStraight => get_straight(dice, 'b'),
        Category::Choice => dice.iter().sum(),
        Category::Yacht => get_yacht(dice),
    }
}

fn collect_scores(dice: Dice) -> [u8; 6] {
    dice.iter().fold([0; 6], |mut scores, num| {
        scores[(*num - 1) as usize] += 1;
        scores
    })
}

fn get_score(dice: Dice, num: usize) -> u8 {
    collect_scores(dice)[num - 1] * num as u8
}

fn get_full_house(dice: Dice) -> u8 {
    let scores = collect_scores(dice);
    let two_of_n = scores.iter().position(|&n| n == 2);
    let three_of_n = scores.iter().position(|&n| n == 3);
    match (two_of_n, three_of_n) {
        (Some(two_index), Some(three_index)) => {
            scores[two_index] * (two_index as u8 + 1)
                + scores[three_index] * (three_index as u8 + 1)
        }
        _ => 0,
    }
}

fn get_poker(dice: Dice) -> u8 {
    let scores = collect_scores(dice);
    match scores.iter().max() {
        Some(5) => (scores.iter().position(|&n| n == 5).unwrap() as u8 + 1) * 4,
        Some(4) => (scores.iter().position(|&n| n == 4).unwrap() as u8 + 1) * 4,
        _ => 0,
    }
}

fn get_straight(dice: Dice, straight_type: char) -> u8 {
    let scores = collect_scores(dice);
    if !scores.iter().all(|&n| n == 0 || n == 1) {
        return 0;
    }
    let index: usize = if straight_type == 'l' { 5 } else { 0 };

    if scores[index] == 0 {
        30
    } else {
        0
    }
}

fn get_yacht(dice: Dice) -> u8 {
    match collect_scores(dice).iter().max() {
        Some(5) => 50,
        _ => 0,
    }
}
