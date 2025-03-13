use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::log::{LogFlag, LogLevel};
use crate::{board::Board, player::Player};

#[derive(Clone, Debug, Default)]
pub struct Game {
    board: Board,
    players: HashMap<Color, Rc<Player>>,
    duration: Option<Duration>,
    log_level: Rc<LogLevel>,
}

impl Game {
    pub fn new(players: HashMap<Color, Rc<Player>>) -> Game {
        Game {
            board: Board::new(),
            players,
            duration: None,
            log_level: Rc::default(),
        }
    }

    pub fn winner(&self) -> Option<Color> {
        self.board.winner()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    #[allow(unused)]
    pub fn set_starting_position(&mut self, board: Board) {
        self.board = board;
    }

    #[allow(unused)]
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    #[allow(unused)]
    pub fn set_log_level(&mut self, log_level: Rc<LogLevel>) {
        self.log_level = log_level;
    }

    #[allow(unused)]
    pub fn play_random_move(&mut self) {
        self.board.play_random_move();
        if self.log_level.is(LogFlag::Position) {
            print!("{}", self.board);
        }
    }

    pub fn play(&mut self) {
        if self.log_level.is(LogFlag::Position) {
            print!("{}", self.board);
        }

        self.play_random_move();

        loop {
            let player = match self.players.get(&self.board.next_color()) {
                Some(player) => player.clone(),
                None => Rc::new(Player::default()),
            };

            // Time
            let start = Instant::now();

            let (x, y) = player.next_move(&self.board);

            // Time
            let duration = start.elapsed();

            // Update the original board with the player's move
            self.board.play(x, y);

            if self.log_level.is(LogFlag::Moves) {
                println!(
                    "{} played ({}, {}) in {:?}",
                    self.board.next_color(),
                    x + 1,
                    y + 1,
                    duration
                );
            }
            if self.log_level.is(LogFlag::Position) {
                println!("{}", self.board);
            }

            if let Some(winner) = self.board.winner() {
                if self.log_level.is(LogFlag::GameResult) {
                    println!("{} wins!", winner);
                }
                break;
            }
        }
    }
}
