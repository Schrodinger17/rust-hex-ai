use std::{rc::Rc, time::Duration};

use crate::{board::Board, strategy::Strategy};

pub struct Player {
    #[allow(unused)]
    pub name: String,
    pub strategy: Rc<dyn Strategy>,
    pub time_by_move: Option<Duration>,
}

impl Player {
    #[allow(unused)]
    pub fn new(name: String, strategy: Rc<dyn Strategy>, time_by_move: Option<Duration>) -> Player {
        Player {
            name,
            strategy,
            time_by_move,
        }
    }

    pub fn next_move(&self, board: &Board) -> (usize, usize) {
        self.strategy.next_move(board, self.time_by_move)
    }
}
