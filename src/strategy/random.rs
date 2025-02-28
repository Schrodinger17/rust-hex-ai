use std::{rc::Rc, time::Duration};

use crate::{board::Board, color::Color, log::LogLevel};
use rand::Rng;

use super::Strategy;

#[derive(Clone)]
pub struct Random;

impl Strategy for Random {
    fn next_move(
        &self,
        board: &Board,
        _color: Color,
        _duration: Option<Duration>,
    ) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..board.size());
        let mut y = rng.gen_range(0..board.size());
        while !board.is_valid(x, y) {
            x = rng.gen_range(0..board.size());
            y = rng.gen_range(0..board.size());
        }
        (x, y)
    }
}

impl Random {
    #[allow(unused)]
    pub fn new() -> Random {
        Random {}
    }
}
