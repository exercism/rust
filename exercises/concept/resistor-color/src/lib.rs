#[derive(Debug, PartialEq)]
pub enum ResistorColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    unimplemented!("convert a color into a numerical representation")
}

pub fn value_to_color_string(value: usize) -> String {
    unimplemented!(
        "convert the value {} into a string representation of color",
        value
    )
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
