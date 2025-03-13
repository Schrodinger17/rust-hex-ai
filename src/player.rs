use std::{rc::Rc, time::Duration};

use crate::{board::Board, strategy::Strategy};

#[derive(Clone)]
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

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "{:?}", self.time_by_move)
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::from("Player") + &rand::random::<u8>().to_string(),
            strategy: Rc::new(crate::strategy::Random::new()),
            time_by_move: None,
        }
    }
}
