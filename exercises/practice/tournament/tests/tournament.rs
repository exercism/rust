#[test]
fn just_the_header_if_no_input() {
    let input = "";
    let expected = "Team                           | MP |  W |  D |  L |  P";

    assert_eq!(tournament::tally(input), expected);
}

#[test]
#[ignore]
fn a_win_is_three_points_a_loss_is_zero_points() {
    let input = "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
        + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";

    assert_eq!(tournament::tally(input), expected);
}

#[test]
#[ignore]
fn a_win_can_also_be_expressed_as_a_loss() {
    let input = "Blithering Badgers;Allegoric Alaskans;loss";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
        + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";

    assert_eq!(tournament::tally(input), expected);
}

#[test]
#[ignore]
fn a_different_team_can_win() {
    let input = "Blithering Badgers;Allegoric Alaskans;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Blithering Badgers             |  1 |  1 |  0 |  0 |  3\n"
        + "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0";

    assert_eq!(tournament::tally(input), expected);
}

#[test]
#[ignore]
fn there_can_be_more_than_one_match() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
        + "Blithering Badgers             |  2 |  0 |  0 |  2 |  0";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn a_draw_is_one_point_each() {
    let input = "Allegoric Alaskans;Blithering Badgers;draw\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  1 |  1 |  0 |  4\n"
        + "Blithering Badgers             |  2 |  0 |  1 |  1 |  1";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn there_can_be_more_than_one_winner() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3\n"
        + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn there_can_be_more_than_two_teams() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Blithering Badgers;Courageous Californians;win\n"
        + "Courageous Californians;Allegoric Alaskans;loss";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
        + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3\n"
        + "Courageous Californians        |  2 |  0 |  0 |  2 |  0";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn typical_input() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Devastating Donkeys;Courageous Californians;draw\n"
        + "Devastating Donkeys;Allegoric Alaskans;win\n"
        + "Courageous Californians;Blithering Badgers;loss\n"
        + "Blithering Badgers;Devastating Donkeys;loss\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n"
        + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
        + "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n"
        + "Courageous Californians        |  3 |  0 |  1 |  2 |  1";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn incomplete_competition_not_all_pairs_have_played() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;draw\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
        + "Blithering Badgers             |  2 |  1 |  1 |  0 |  4\n"
        + "Courageous Californians        |  2 |  0 |  1 |  1 |  1\n"
        + "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn ties_broken_alphabetically() {
    let input = "Courageous Californians;Devastating Donkeys;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win\n"
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;win\n"
        + "Blithering Badgers;Devastating Donkeys;draw\n"
        + "Allegoric Alaskans;Courageous Californians;draw";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7\n"
        + "Courageous Californians        |  3 |  2 |  1 |  0 |  7\n"
        + "Blithering Badgers             |  3 |  0 |  1 |  2 |  1\n"
        + "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1";

    assert_eq!(tournament::tally(&input), expected);
}
