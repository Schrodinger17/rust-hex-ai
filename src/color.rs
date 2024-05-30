use crate::score::Score;
use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
    None,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::None => Color::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
            Color::None => "None".to_string(),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Color::Black => '○',
            Color::White => '●',
            Color::None => '.',
        }
    }

    pub fn win_score(&self) -> Score {
        match self {
            Color::White => Score::WhiteCheckMate,
            Color::Black => Score::BlackCheckMate,
            _ => Score::Advantage(0.0),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "None"),
            Color::None => write!(f, "White"),
        }
    }
}