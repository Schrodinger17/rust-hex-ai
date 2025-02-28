use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::{board::Board, player::Player};

pub struct Game {
    board: Board,
    players: HashMap<Color, Rc<Player>>,
    turn: Color,
    duration: Option<Duration>,
    winner: Color,
    display: bool,
}

impl Game {
    pub fn new(board_size: usize, players: HashMap<Color, Rc<Player>>) -> Game {
        Game {
            board: Board::new(board_size),
            players,
            turn: Color::White,
            duration: None,
            winner: Color::None,
            display: true,
        }
    }

    pub fn winner(&self) -> Color {
        self.winner
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    #[allow(dead_code)]
    pub fn set_starting_position(&mut self, board: Board) {
        self.board = board;
    }

    #[allow(dead_code)]
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    #[allow(dead_code)]
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    #[allow(dead_code)]
    pub fn play_random_move(&mut self) {
        self.board.play_random_move(self.turn);
        self.turn = self.turn.opponent();
        if self.display {
            print!("{}", self.board);
        }
    }

    pub fn play(&mut self) {
        if self.display {
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

            if self.display {
                println!(
                    "{} played ({}, {}) in {:?}",
                    self.turn,
                    x + 1,
                    y + 1,
                    duration
                );
                println!("{}", self.board);
            }

            if self.board.is_win(self.turn) || self.board.is_full() {
                self.winner = self.turn;
                if self.display {
                    println!("{} wins!", self.winner);
                }
                break;
            }

            self.turn = self.turn.opponent();
        }
    }
}
