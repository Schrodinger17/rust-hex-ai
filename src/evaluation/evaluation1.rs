use super::Evaluation;
use crate::{board::Board, color::Color, score::Score};

#[derive(Clone)]
pub struct Evaluation1;

impl Evaluation for Evaluation1 {
    fn score(&self, board: &Board) -> Score {
        match (
            board.missing_move_to_win(Color::Black),
            board.missing_move_to_win(Color::White),
        ) {
            (None, _) => Score::WhiteCheckMate,
            (_, None) => Score::BlackCheckMate,
            (Some(missing_move_to_win_black), Some(missing_move_to_win_white)) => Score::Advantage(
                missing_move_to_win_black as f64 - missing_move_to_win_white as f64,
            ),
        }
    }
}

impl Evaluation1 {
    pub fn new() -> Evaluation1 {
        Evaluation1 {}
    }
}

impl Default for Evaluation1 {
    fn default() -> Self {
        Self::new()
    }
}

//test
#[cfg(test)]

mod tests {
    use super::*;
    use crate::board::Board;
    use crate::color::Color;

    #[test]
    #[ignore]
    fn test_evaluation1() {
        let board = Board::new();
        let evaluation = Evaluation1::new();
        assert_eq!(evaluation.score(&board), Score::Advantage(0.0));

        let mut board = Board::new();
        board.set(0, 0, Color::Black);
        assert_eq!(evaluation.score(&board), Score::Advantage(-1.0));

        let mut board = Board::new();
        board.set(0, 0, Color::White);
        assert_eq!(evaluation.score(&board), Score::Advantage(1.0));

        let mut board = Board::new();
        board.set(0, 0, Color::Black);
        board.set(1, 0, Color::Black);
        board.set(2, 0, Color::Black);
        assert_eq!(evaluation.score(&board), Score::BlackCheckMate);

        let mut board = Board::new();
        board.set(0, 0, Color::White);
        board.set(0, 1, Color::White);
        board.set(0, 2, Color::White);
        assert_eq!(evaluation.score(&board), Score::WhiteCheckMate);
    }
}
