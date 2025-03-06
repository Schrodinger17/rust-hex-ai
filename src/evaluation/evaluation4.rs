use super::Evaluation;
use crate::{board::Board, color::Color, distance::Distance, score::Score};

#[derive(Clone)]
pub struct Evaluation4;

impl Evaluation for Evaluation4 {
    fn score(&self, board: &Board) -> Score {
        fn calculate(vec: Vec<f64>) -> f64 {
            vec.iter().map(|d| d * d).sum::<f64>() / vec.iter().sum::<f64>()
        }

        match (
            board.missing_move_to_win2(Color::Black),
            board.missing_move_to_win2(Color::White),
        ) {
            (None, _) => Score::WhiteCheckMate,
            (_, None) => Score::BlackCheckMate,
            (Some(_), Some(_)) => {
                let dist_matrix_white = board.get_dist_matrix(Color::White);
                let dist_white: Vec<f64> = dist_matrix_white
                    .iter()
                    .map(|row| row.last().unwrap())
                    .filter(|&&d| matches!(d, Distance::Reachable(_)))
                    .map(|&d| Option::<usize>::from(d).unwrap() as f64)
                    .collect();

                let score_white: f64 = calculate(dist_white);

                let dist_matrix_black = board.get_dist_matrix(Color::Black);
                let dist_black: Vec<f64> = dist_matrix_black
                    .last()
                    .unwrap()
                    .iter()
                    .filter(|&&d| matches!(d, Distance::Reachable(_)))
                    .map(|&d| Option::<usize>::from(d).unwrap() as f64)
                    .collect();

                let score_black: f64 = calculate(dist_black);

                Score::Advantage(score_white - score_black)
            }
        }
    }
}

impl Evaluation4 {
    pub fn new() -> Evaluation4 {
        Evaluation4 {}
    }
}

impl Default for Evaluation4 {
    fn default() -> Self {
        Self::new()
    }
}
