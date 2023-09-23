use std::{time::Duration, sync::Arc};

use crate::{strategy::Strategy, board::Board, color::Color};

pub struct Player {
    pub name: String,
    pub strategy: Arc<dyn Strategy>,
    pub time_by_move: Option<Duration>,
}

impl Player {
    pub fn new(name: String, strategy: Arc<dyn Strategy>, time_by_move: Option<Duration>) -> Player {
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