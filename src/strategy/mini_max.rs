use std::{time::Duration, sync::Arc};

use crate::{board::Board, evaluation::Evaluation, color::Color};

use super::StrategyTrait;


#[derive(Clone)]
pub struct MiniMax {
    duration: Option<Duration>,
    max_depth: usize,
    evaluation: Arc<dyn Evaluation>,
}

impl StrategyTrait for MiniMax {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.minimax(&board, color, self.max_depth,  self.duration),
            Some(duration) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.minimax(&board, color, depth,  self.duration);
                while time.elapsed() < duration && depth < self.max_depth {
                    depth += 1;
                    best_move = self.minimax(&board, color, depth,  self.duration);
                }
                println!("Depth: {} in {:?}", depth, Duration::from_millis(time.elapsed().as_millis() as u64));
                return best_move;
            },
        }
    }
}

impl MiniMax {
    #[allow(dead_code)]
    pub fn new(evaluation: Arc<dyn Evaluation>, max_depth: usize, duration: Option<Duration>) -> MiniMax {
        MiniMax {
            evaluation,
            max_depth,
            duration,
        }
    }

    #[allow(dead_code)]
    pub fn set_duration(&mut self, duration: Option<Duration>) {
        self.duration = duration;
    }

    #[allow(dead_code)]
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }

    fn minimax(&self, board: &Board, color: Color, depth: usize, duration: Option<Duration>) -> (usize, usize) {
        match self._minimax(board, color, depth, duration) {
            (_, Some((x, y))) => (x, y),
            _ => panic!("Error in minimax"),
        }
    }

    fn _minimax(&self, board: &Board, color: Color, depth: usize, duration: Option<Duration>) -> (f64, Option<(usize, usize)>) {
        if board.is_win(color.opponent()) {
            return (color.opponent().win_score(), None);
        }

        if depth == 0 {
            return (self.evaluation.score(board), None);
        }

        let mut best_score: f64 = color.opponent().win_score();
        let mut best_move = board.a_possible_move();

        for (x, y) in board.possible_moves() {
            let mut new_board = board.clone();
            new_board.set(x, y, color);
            let (score, _) = self._minimax(&new_board, color.opponent(), depth - 1, duration);
            if color == Color::White {
                if score > best_score {
                    best_score = score;
                    best_move = (x, y);
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = (x, y);
                }
            }

            if depth == self.max_depth {
                //println!("{} {} {}", x+1, y+1, score); // TODO: remove this debug print
            }
        }

        /*
        if depth == self.max_depth {
            println!("Board score : {}", self.evaluation.score(board)); // TODO: remove this debug print
            println!("Deep score : {}", best_score); // TODO: remove this debug print
            println!("Best move : {:?}", (best_move.0 + 1, best_move.1 + 1)); // TODO: remove this debug print
        }
        */
        
        (best_score, Some(best_move))
    }
}