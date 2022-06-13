use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, IntEnum, Sequence)]
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
    color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::from_int(value)
        .map(|color| format!("{:?}", color))
        .unwrap_or_else(|_| "value out of range".into())
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
