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

    pub fn win_score(&self) -> f64 {
        match self {
            Color::White => f64::MAX,
            Color::Black => f64::MIN,
            Color::None => 0.0,
        }
    }
}