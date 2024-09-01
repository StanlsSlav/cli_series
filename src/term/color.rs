use crate::term::CSI;
use std::fmt::Display;

pub(crate) const FOREGROUND: i32 = 38;
pub(crate) const BACKGROUND: i32 = 48;

pub(crate) enum Color {
    Rgb(u8, u8, u8),
    Red,
    Green,
    Blue,
    Magenta,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{CSI}{FOREGROUND};{}m",
            match self {
                Color::Rgb(r, g, b) => format!("{};{};{}", r, g, b),
                Color::Red => "255;0;0".to_string(),
                Color::Green => "0;255;0".to_string(),
                Color::Blue => "0;0;255".to_string(),
                Color::Magenta => "170;0;170".to_string(),
            }
        )
    }
}

pub(crate) struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}
