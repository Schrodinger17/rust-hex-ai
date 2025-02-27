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

//test
#[cfg(test)]

mod tests {
    use super::*;
    use crate::board::Board;
    use crate::color::Color;

    #[test]
    fn test_evaluation2() {
        let board = Board::new(3);
        let evaluation = Evaluation2::new();
        assert_eq!(evaluation.score(&board), Score::Advantage(0.0));

        let mut board = Board::new(3);
        board.set(0, 0, Color::Black);
        assert_eq!(evaluation.score(&board), Score::Advantage(-1.0));

        let mut board = Board::new(3);
        board.set(0, 0, Color::White);
        assert_eq!(evaluation.score(&board), Score::Advantage(1.0));

        let mut board = Board::new(3);
        board.set(0, 0, Color::Black);
        board.set(1, 0, Color::Black);
        board.set(2, 0, Color::Black);
        assert_eq!(evaluation.score(&board), Score::BlackCheckMate);

        let mut board = Board::new(3);
        board.set(0, 0, Color::White);
        board.set(0, 1, Color::White);
        board.set(0, 2, Color::White);
        assert_eq!(evaluation.score(&board), Score::WhiteCheckMate);
    }
}
