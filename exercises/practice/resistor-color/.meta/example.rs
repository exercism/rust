use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
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

pub fn color_to_value(color: ResistorColor) -> u8 {
    color as u8
}

pub fn value_to_color_string(value: u8) -> String {
    ResistorColor::from_int(value).map(|color| format!("{:?}", color)).unwrap_or("value out of range".into())
}

pub fn colors() -> Vec<ResistorColor> {
    (0..=9).map(|i| ResistorColor::from_int(i).unwrap_or(ResistorColor::Black)).collect()
}
