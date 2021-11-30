use resistor_color::{color_to_value, colors, value_to_color_string, ResistorColor};

#[test]
fn test_black() {
    assert_eq!(color_to_value(ResistorColor::Black), 0);
}

#[test]
#[ignore]
fn test_orange() {
    assert_eq!(color_to_value(ResistorColor::Orange), 3);
}

#[test]
#[ignore]
fn test_white() {
    assert_eq!(color_to_value(ResistorColor::White), 9);
}

#[test]
#[ignore]
fn test_2() {
    assert_eq!(value_to_color_string(2), String::from("Red"));
}

#[test]
#[ignore]
fn test_6() {
    assert_eq!(value_to_color_string(6), String::from("Blue"));
}

#[test]
#[ignore]
fn test_8() {
    assert_eq!(value_to_color_string(8), String::from("Grey"));
}

#[test]
#[ignore]
fn test_11_out_of_range() {
    assert_eq!(
        value_to_color_string(11),
        String::from("value out of range")
    );
}

#[test]
#[ignore]
fn test_all_colors() {
    use ResistorColor::*;
    assert_eq!(
        colors(),
        vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
    );
}
