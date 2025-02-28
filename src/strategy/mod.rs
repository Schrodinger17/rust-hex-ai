mod alpha_beta;
mod alpha_beta_2;
mod alpha_beta_3;
mod alpha_beta_4;
mod human;
mod mcts;
mod mini_max;
mod random;

pub use alpha_beta::AlphaBeta;
pub use alpha_beta_2::AlphaBeta2;
pub use alpha_beta_4::AlphaBeta4;
pub use human::Human;
pub use mcts::MCTS;
pub use mini_max::MiniMax;
pub use random::Random;

use crate::{board::Board, color::Color, log::LogLevel};

use std::{rc::Rc, time::Duration};

pub trait Strategy {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize);
}
