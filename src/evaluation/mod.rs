pub(crate) mod evaluation1;

use crate::board::Board;

use self::evaluation1::Evaluation1;

pub trait EvaluationTrait {
    fn score(&self, board: &Board) -> f64;
}

#[warn(dead_code)]

#[derive(Clone)]
pub enum Evaluation {
    Evaluation1(Evaluation1),
}

impl EvaluationTrait for Evaluation {
    fn score(&self, board: &Board) -> f64 {
        match self {
            Evaluation::Evaluation1(evaluation) => evaluation.score(board),
        }
    }
}
