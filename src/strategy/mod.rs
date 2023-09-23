pub(crate) mod human;
pub(crate) mod random;
pub(crate) mod mini_max;
pub(crate) mod alpha_beta;
pub(crate) mod alpha_beta_2;

use crate::{board::Board, color::Color};

use std::time::Duration;

pub trait Strategy {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize);
}
