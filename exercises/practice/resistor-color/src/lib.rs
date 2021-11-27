#[derive(Debug, PartialEq)]
pub enum ResistorColor {
    White,
    Grey,
    Violet,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Brown,
    Black,
}

pub fn color_to_value(_color: ResistorColor) -> u8 {
    unimplemented!("convert a color into a numerical representation")
}

pub fn value_to_color_string(value: u8) -> String {
    unimplemented!("convert the value {} into a string representation of color",
value)
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors in order")
}
