use std::time::Duration;

use crate::{board::Board, evaluation::EvaluationTrait, color::Color};

use super::StrategyTrait;

use crate::evaluation::Evaluation;


#[derive(Clone)]
pub struct AlphaBeta {
    duration: Option<Duration>,
    max_depth: usize,
    evaluation: Evaluation,
}

impl StrategyTrait for AlphaBeta {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.alpha_beta(&board, color, self.max_depth,  self.duration),
            Some(duration) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.alpha_beta(&board, color, depth,  self.duration);
                while time.elapsed() < duration && depth < self.max_depth {
                    depth += 1;
                    best_move = self.alpha_beta(&board, color, depth,  self.duration);
                }
                println!("Depth: {} in {:?}", depth, Duration::from_millis(time.elapsed().as_millis() as u64));
                return best_move;
            },
        }
    }
}

impl AlphaBeta {
    pub fn new(evaluation: Evaluation, max_depth: usize, duration: Option<Duration>) -> AlphaBeta {
        AlphaBeta {
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

    fn alpha_beta(&self, board: &Board, color: Color, depth: usize, duration: Option<Duration>) -> (usize, usize) {
        match self._alpha_beta(board, color, depth, f64::MIN, f64::MAX, duration) {
            (_, Some((x, y))) => (x, y),
            _ => panic!("Error in alpha_beta"),
        }
    }

    fn _alpha_beta(&self, board: &Board, color: Color, depth: usize, alpha: f64, beta: f64, duration: Option<Duration>) -> (f64, Option<(usize, usize)>) {
        let mut alpha = alpha;
        let mut beta = beta;
        
        if board.is_win(color) {
            return (color.win_score(), None);
        } else if board.is_win(color.opponent()) {
            return (color.opponent().win_score(), None);
        }

        if depth == 0 {
            return (self.evaluation.score(board), None);
        }

        let mut value: f64;
        let mut best_move = board.a_possible_move();
        let possible_moves = board.possible_moves();

        if color == Color::White {
            value = f64::MIN;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.set(x, y, color);
                
                let (score, _) = self._alpha_beta(&new_board, color.opponent(), depth - 1, alpha, beta, duration);
                
                if score > value {
                    value = score;
                    best_move = (x, y);
                }

                if value > alpha {
                    alpha = value;
                }
                
                if value >= beta {
                    break;
                }

                if depth == self.max_depth {
                    //println!("{} {} {}", x+1, y+1, score); // TODO: remove this debug print
                }
            }
        } else {
            value = f64::MAX;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.set(x, y, color);
                let (score, _) = self._alpha_beta(&new_board, color.opponent(), depth - 1,  alpha, beta, duration);
                if score < value {
                    value = score;
                    best_move = (x, y);
                }
                
                if value < beta {
                    beta = value;
                }

                if value <= alpha {
                    break;
                }

                if depth == self.max_depth {
                    //println!("{} {} {}", x+1, y+1, score); // TODO: remove this debug print
                }
            }
        }
        /*
        if depth == self.max_depth {
            println!("Board score : {}", self.evaluation.score(board)); // TODO: remove this debug print
            println!("Deep score : {}", value); // TODO: remove this debug print
            println!("Best move : {:?}", (best_move.0 + 1, best_move.1 + 1)); // TODO: remove this debug print
        }*/

        (value, Some(best_move))
    }
}