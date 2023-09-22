mod human;
pub(crate) mod random;
pub(crate) mod mini_max;
pub(crate) mod alpha_beta;
pub(crate) mod alpha_beta_2;

use crate::{board::Board, color::Color};

use std::time::Duration;

use self::{random::Random, human::Human, mini_max::MiniMax, alpha_beta::AlphaBeta, alpha_beta_2::AlphaBeta2};

pub trait StrategyTrait {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize);
}

#[warn(dead_code)]
#[derive(Clone)]
pub enum Strategy {
    #[allow(dead_code)]
    Human(Human),
    #[allow(dead_code)]
    Random(Random),
    #[allow(dead_code)]
    MiniMax(MiniMax),
    #[allow(dead_code)]
    AlphaBetaV2(AlphaBeta2),
    #[allow(dead_code)]
    AlphaBeta(AlphaBeta),
}

impl StrategyTrait for Strategy {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        match self {
            Strategy::Human(human) => human.next_move(board, color, duration),
            Strategy::Random(random) => random.next_move(board, color, duration),
            Strategy::MiniMax(mini_max) => mini_max.next_move(board, color, duration),
            Strategy::AlphaBeta(alpha_beta) => alpha_beta.next_move(board, color, duration),
            Strategy::AlphaBetaV2(alpha_beta_2) => alpha_beta_2.next_move(board, color, duration),
        }
    }
}
