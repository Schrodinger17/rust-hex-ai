use super::Evaluation;
use crate::{board::Board, color::Color, score::Score};

#[derive(Clone)]
pub struct Evaluation1;

impl Evaluation for Evaluation1 {
    fn score(&self, board: &Board) -> Score {
        let missing_move_to_win_black = board.missing_move_to_win(Color::Black) as f64;
        if missing_move_to_win_black == 0.0 {
            return Score::BlackCheckMate;
        }

        let missing_move_to_win_white = board.missing_move_to_win(Color::White) as f64;
        if missing_move_to_win_white == 0.0 {
            return Score::WhiteCheckMate;
        }

        Score::Advantage(missing_move_to_win_black - missing_move_to_win_white)
    }
}

impl Evaluation1 {
    pub fn new() -> Evaluation1 {
        Evaluation1 {}
    }
}

//test
#[cfg(test)]

mod tests {
    use super::*;
    use crate::board::Board;
    use crate::color::Color;

    #[test]
    fn test_evaluation1() {
        let board = Board::new(3);
        let evaluation = Evaluation1::new();
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
