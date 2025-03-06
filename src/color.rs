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

    pub fn to_char(self) -> char {
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

impl Iterator for Color {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        *self = match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::None => Color::None,
        };
        if *self == Color::None {
            None
        } else {
            Some(self.clone())
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::White => write!(f, "White"),
            Color::None => write!(f, "None"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let mut color = Color::White;
        assert_eq!(color.next().unwrap(), Color::Black);
        assert_eq!(color.next().unwrap(), Color::White);
        assert_eq!(color.next().unwrap(), Color::Black);
    }
}
