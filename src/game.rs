use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::log::{LogFlag, LogLevel};
use crate::{board::Board, player::Player};

pub struct Game {
    board: Board,
    players: HashMap<Color, Rc<Player>>,
    turn: Color,
    duration: Option<Duration>,
    winner: Color,
    log_level: Rc<LogLevel>,
}

impl Game {
    pub fn new(board_size: usize, players: HashMap<Color, Rc<Player>>) -> Game {
        Game {
            board: Board::new(board_size),
            players,
            turn: Color::White,
            duration: None,
            winner: Color::None,
            log_level: Rc::default(),
        }
    }

    pub fn winner(&self) -> Color {
        self.winner
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
        self.board.play_random_move(self.turn);
        self.turn = self.turn.opponent();
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
            let player = self.players.get(&self.turn).unwrap();

            // Clone the board before passing it to next_move
            let cloned_board = self.board.clone();

            // Time
            let start = Instant::now();

            let (x, y) = player.next_move(&cloned_board, self.turn);

            // Time
            let duration = start.elapsed();

            // Update the original board with the player's move
            self.board.set(x, y, self.turn);

            if self.log_level.is(LogFlag::Moves) {
                println!(
                    "{} played ({}, {}) in {:?}",
                    self.turn,
                    x + 1,
                    y + 1,
                    duration
                );
            }
            if self.log_level.is(LogFlag::Position) {
                println!("{}", self.board);
            }

            if self.board.is_win(self.turn) || self.board.is_full() {
                self.winner = self.turn;
                if self.log_level.is(LogFlag::GameResult) {
                    println!("{} wins!", self.winner);
                }
                break;
            }

            self.turn = self.turn.opponent();
        }
    }
}
