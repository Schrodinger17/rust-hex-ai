use super::Evaluation;
use crate::{board::Board, color::Color, score::Score};

#[derive(Clone)]
pub struct Evaluation2;

impl Evaluation for Evaluation2 {
    fn score(&self, board: &Board) -> Score {
        match (
            board.missing_move_to_win2(Color::Black),
            board.missing_move_to_win2(Color::White),
        ) {
            (None, _) => Score::WhiteCheckMate,
            (_, None) => Score::BlackCheckMate,
            (Some(missing_move_to_win_black), Some(missing_move_to_win_white)) => Score::Advantage(
                missing_move_to_win_black as f64 - missing_move_to_win_white as f64,
            ),
        }
    }
}

impl Evaluation2 {
    pub fn new() -> Evaluation2 {
        Evaluation2 {}
    }
}

impl Default for Evaluation2 {
    fn default() -> Self {
        Self::new()
    }
}
