use std::time::Duration;

use crate::{strategy::{Strategy, StrategyTrait}, board::Board, color::Color};

pub struct Player {
    pub name: String,
    pub strategy: Strategy,
    pub time_by_move: Option<Duration>,
}

impl Player {
    pub fn new(name: String, strategy: Strategy, time_by_move: Option<Duration>) -> Player {
        Player {
            name,
            strategy,
            time_by_move,
        }
    }

    pub fn next_move(&self, board: &Board, color: Color) -> (usize, usize) {
        self.strategy.next_move(board, color, self.time_by_move)
    }
}