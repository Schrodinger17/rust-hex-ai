pub(crate) mod alpha_beta;
pub(crate) mod alpha_beta_2;
pub(crate) mod alpha_beta_3;
pub(crate) mod human;
pub(crate) mod mini_max;
pub(crate) mod random;

use crate::{board::Board, color::Color};

use std::time::Duration;

pub trait Strategy {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize);
}
