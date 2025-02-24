mod evaluation1;

pub use evaluation1::Evaluation1;

use crate::{board::Board, score::Score};

pub trait Evaluation {
    fn score(&self, board: &Board) -> Score;
}
