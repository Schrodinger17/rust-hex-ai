use super::Evaluation;
use crate::{board::Board, color::Color, score::Score};

#[derive(Clone)]
pub struct Evaluation3;

impl Evaluation for Evaluation3 {
    fn score(&self, board: &Board) -> Score {
        match (
            board.missing_move_to_win2(Color::Black),
            board.missing_move_to_win2(Color::White),
        ) {
            (None, _) => Score::WhiteCheckMate,
            (_, None) => Score::BlackCheckMate,
            (Some(missing_move_to_win_black), Some(missing_move_to_win_white)) => {
                match board.next_player() {
                    Color::Black => Score::Advantage(
                        missing_move_to_win_black as f64 - 2.0 * missing_move_to_win_white as f64,
                    ),
                    Color::White => Score::Advantage(
                        2.0 * missing_move_to_win_black as f64 - missing_move_to_win_white as f64,
                    ),
                    Color::None => unreachable!(),
                }
            }
        }
    }
}

impl Evaluation3 {
    pub fn new() -> Evaluation3 {
        Evaluation3 {}
    }
}

impl Default for Evaluation3 {
    fn default() -> Self {
        Self::new()
    }
}
