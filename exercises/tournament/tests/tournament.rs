extern crate tournament;

#[test]
fn test_good() {
    let input = "Allegoric Alaskians;Blithering Badgers;win\n".to_string() +
        "Devastating Donkeys;Courageous Californians;draw\n" +
        "Devastating Donkeys;Allegoric Alaskians;win\n" +
        "Courageous Californians;Blithering Badgers;loss\n" +
        "Blithering Badgers;Devastating Donkeys;loss\n" +
        "Allegoric Alaskians;Courageous Californians;win";
    let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string() +
        "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n" +
        "Allegoric Alaskians            |  3 |  2 |  0 |  1 |  6\n" +
        "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n" +
        "Courageous Californians        |  3 |  0 |  1 |  2 |  1";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn test_ignore_bad_lines() {
    let input = "Allegoric Alaskians;Blithering Badgers;win\n".to_string() +
        "Devastating Donkeys_Courageous Californians;draw\n" +
        "Devastating Donkeys;Allegoric Alaskians;win\n" +
        "\n" +
        "Courageous Californians;Blithering Badgers;loss\n" +
        "Bla;Bla;Bla\n" +
        "Blithering Badgers;Devastating Donkeys;loss\n" +
        "# Yackity yackity yack\n" +
        "Allegoric Alaskians;Courageous Californians;win\n" +
        "Devastating Donkeys;Courageous Californians;draw\n" +
        "Devastating Donkeys@Courageous Californians;draw";
    let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string() +
        "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n" +
        "Allegoric Alaskians            |  3 |  2 |  0 |  1 |  6\n" +
        "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n" +
        "Courageous Californians        |  3 |  0 |  1 |  2 |  1";

    assert_eq!(tournament::tally(&input), expected);
}

#[test]
#[ignore]
fn test_incomplete_competition() {
    let input = "Allegoric Alaskians;Blithering Badgers;win\n".to_string() +
        "Devastating Donkeys;Allegoric Alaskians;win\n" +
        "Courageous Californians;Blithering Badgers;loss\n" +
        "Allegoric Alaskians;Courageous Californians;win";
    let expected = "Team                           | MP |  W |  D |  L |  P\n".to_string() +
        "Allegoric Alaskians            |  3 |  2 |  0 |  1 |  6\n" +
        "Blithering Badgers             |  2 |  1 |  0 |  1 |  3\n" +
        "Devastating Donkeys            |  1 |  1 |  0 |  0 |  3\n" +
        "Courageous Californians        |  2 |  0 |  0 |  2 |  0";

    assert_eq!(tournament::tally(&input), expected);
}
