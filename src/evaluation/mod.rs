mod evaluation1;
mod evaluation2;
mod evaluation3;
mod evaluation4;

pub use evaluation1::Evaluation1;
pub use evaluation2::Evaluation2;
pub use evaluation3::Evaluation3;
pub use evaluation4::Evaluation4;

use crate::{board::Board, score::Score};

pub trait Evaluation {
    fn score(&self, board: &Board) -> Score;
}
