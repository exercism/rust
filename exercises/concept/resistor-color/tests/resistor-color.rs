use resistor_color::{color_to_value, colors, value_to_color_string, ResistorColor};

#[test]
fn black() {
    assert_eq!(color_to_value(ResistorColor::Black), 0);
}

#[test]
#[ignore]
fn orange() {
    assert_eq!(color_to_value(ResistorColor::Orange), 3);
}

#[test]
#[ignore]
fn white() {
    assert_eq!(color_to_value(ResistorColor::White), 9);
}

#[test]
#[ignore]
fn two() {
    assert_eq!(value_to_color_string(2), String::from("Red"));
}

#[test]
#[ignore]
fn six() {
    assert_eq!(value_to_color_string(6), String::from("Blue"));
}

#[test]
#[ignore]
fn eight() {
    assert_eq!(value_to_color_string(8), String::from("Grey"));
}

#[test]
#[ignore]
fn eleven_out_of_range() {
    assert_eq!(
        value_to_color_string(11),
        String::from("value out of range")
    );
}

#[test]
#[ignore]
fn all_colors() {
    use ResistorColor::*;
    assert_eq!(
        colors(),
        vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
    );
}
