pub(crate) mod evaluation1;

use crate::board::Board;

pub trait Evaluation {
    fn score(&self, board: &Board) -> f64;
}
