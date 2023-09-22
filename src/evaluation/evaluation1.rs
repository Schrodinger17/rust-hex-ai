use super::EvaluationTrait;
use crate::{board::Board, color::Color};


#[derive(Clone)]
pub struct Evaluation1;

impl EvaluationTrait for Evaluation1 {
    fn score(&self, board: &Board) -> f64 {
        board.missing_move_to_win(Color::Black) as f64 - board.missing_move_to_win(Color::White) as f64
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
        assert_eq!(evaluation.score(&board), 0.0);

        let mut board = Board::new(3);
        board.set(0, 0, Color::Black);
        assert_eq!(evaluation.score(&board), -1.0);

        let mut board = Board::new(3);
        board.set(0, 0, Color::White);
        assert_eq!(evaluation.score(&board), 1.0);
    }
}