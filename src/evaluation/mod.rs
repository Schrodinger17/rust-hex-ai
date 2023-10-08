pub(crate) mod evaluation1;

use crate::{board::Board, score::Score};

pub trait Evaluation {
    fn score(&self, board: &Board) -> Score;
}
