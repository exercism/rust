use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::into_enum_iter()
        .nth(value)
        .map(|color| format!("{:?}", color))
        .unwrap_or_else(|| "value out of range".into())
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
