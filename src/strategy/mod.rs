mod alpha_beta;
mod alpha_beta_2;
mod alpha_beta_3;
mod alpha_beta_4;
mod human;
mod mini_max;
mod random;
//mod mcts;

pub use alpha_beta::AlphaBeta;
pub use alpha_beta_2::AlphaBeta2;
pub use alpha_beta_4::AlphaBeta4;
pub use human::Human;
pub use mini_max::MiniMax;
pub use random::Random;
//pub use mcts::MCTS;

use crate::{board::Board, color::Color, log::LogLevel};

use std::{rc::Rc, time::Duration};

pub trait Strategy {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize);
}
