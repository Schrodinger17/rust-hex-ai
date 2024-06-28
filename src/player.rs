use std::{rc::Rc, time::Duration};

use crate::{board::Board, color::Color, strategy::Strategy};

pub struct Player {
    #[allow(dead_code)]
    pub name: String,
    pub strategy: Rc<dyn Strategy>,
    pub time_by_move: Option<Duration>,
}

impl Player {
    pub fn new(name: String, strategy: Rc<dyn Strategy>, time_by_move: Option<Duration>) -> Player {
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
