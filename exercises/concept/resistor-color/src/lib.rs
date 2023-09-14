#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    todo!("convert color {color:?} into a numerical representation")
}

pub fn value_to_color_string(value: u32) -> String {
    todo!("convert the value {value} into a string representation of color")
}

pub fn colors() -> Vec<ResistorColor> {
    todo!("return a list of all the colors ordered by resistance")
}
