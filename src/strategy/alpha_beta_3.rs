use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::{board::Board, color::Color, evaluation::Evaluation};

use super::Strategy;

#[derive(Clone)]
pub struct AlphaBeta3 {
    duration: Option<Duration>,
    max_depth: usize,
    evaluation: Arc<dyn Evaluation>,
}

impl Strategy for AlphaBeta3 {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.alpha_beta(&board, color, self.max_depth, self.duration),
            Some(duration) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.alpha_beta(&board, color, depth, self.duration);
                while time.elapsed() < duration && depth < self.max_depth {
                    depth += 1;
                    best_move = self.alpha_beta(&board, color, depth, self.duration);
                }
                println!(
                    "Depth: {} in {:?}",
                    depth,
                    Duration::from_millis(time.elapsed().as_millis() as u64)
                );
                return best_move;
            }
        }
    }
}

impl AlphaBeta3 {
    #[allow(dead_code)]
    pub fn new(
        evaluation: Arc<dyn Evaluation>,
        max_depth: usize,
        duration: Option<Duration>,
    ) -> AlphaBeta3 {
        AlphaBeta3 {
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

    fn alpha_beta(
        &self,
        board: &Board,
        color: Color,
        depth: usize,
        duration: Option<Duration>,
    ) -> (usize, usize) {
        #[allow(unused_mut)]
        let mut score_dict = HashMap::new();
        match self._alpha_beta(
            board,
            color,
            depth,
            f64::MIN,
            f64::MAX,
            duration,
            &mut score_dict,
        ) {
            (_, Some((x, y))) => (x, y),
            _ => panic!("Error in alpha_beta"),
        }
    }

    fn get_score(&self, board: &Board, score_dict: &mut HashMap<Board, f64>) -> f64 {
        if score_dict.contains_key(&board) {
            return score_dict[&board];
        } else {
            return self.evaluation.score(&board);
        }
    }

    fn possible_moves_sorted(
        &self,
        board: &Board,
        color: Color,
        score_dict: &mut HashMap<Board, f64>,
    ) -> Vec<(usize, usize)> {
        let mut s_moves = board
            .possible_moves()
            .iter()
            .map(|(x, y)| {
                let mut board = board.clone();
                board.set(*x, *y, color);
                let score = self.get_score(&board, score_dict);
                //println!("{} {} {}", x+1, y+1, score);
                ((*x, *y), score)
            })
            .collect::<Vec<((usize, usize), f64)>>();

        s_moves.sort_by(|(_, score_a), (_, score_b)| score_a.partial_cmp(&score_b).unwrap());

        if let Color::White = color {
            s_moves.reverse();
        }

        s_moves
            .iter()
            .map(|((x, y), _)| (*x, *y))
            .collect::<Vec<(usize, usize)>>()
    }

    fn keep_bests_moves(&self, board: &Board, possible_moves: Vec<(usize, usize)>) -> Vec<(usize, usize)>{
        let nb_max_moves = board.size() * board.size() / 5;
        if possible_moves.len() > nb_max_moves {
            return possible_moves[0..nb_max_moves].to_vec();
        }
        possible_moves
    }

    fn _alpha_beta(
        &self,
        board: &Board,
        color: Color,
        depth: usize,
        alpha: f64,
        beta: f64,
        duration: Option<Duration>,
        score_dict: &mut HashMap<Board, f64>,
    ) -> (f64, Option<(usize, usize)>) {
        let mut alpha = alpha;
        let mut beta = beta;

        if board.is_win(color) {
            return (color.win_score(), None);
        } else if board.is_win(color.opponent()) {
            return (color.opponent().win_score(), None);
        }

        if depth == 0 {
            return (self.get_score(board, score_dict), None);
        }

        let mut value: f64;
        let mut best_move = board.a_possible_move();
        let mut possible_moves = self.possible_moves_sorted(board, color, score_dict);

        possible_moves = self.keep_bests_moves(board, possible_moves);

        if color == Color::White {
            value = f64::MIN;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.set(x, y, color);

                let score = match score_dict.get(&new_board) {
                    Some(score) => {
                        score.clone()
                    }
                    None => {
                        self._alpha_beta(
                            &new_board,
                            color.opponent(),
                            depth - 1,
                            alpha,
                            beta,
                            duration,
                            score_dict,
                        ).0
                    }
                };

                if score > value {
                    value = score;
                    score_dict.insert( new_board.clone(), score);
                    best_move = (x, y);
                }

                if value > alpha {
                    alpha = value;
                }

                if value >= beta {
                    break;
                }

                if depth == self.max_depth {
                    println!("{} {} {} {}", x+1, y+1, score, depth); // TODO: remove this debug print
                }
            }
        } else {
            value = f64::MAX;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.set(x, y, color);

                let score = match score_dict.get(&new_board) {
                    Some(score) => {
                        score.clone()
                    }
                    None => {
                        self._alpha_beta(
                            &new_board,
                            color.opponent(),
                            depth - 1,
                            alpha,
                            beta,
                            duration,
                            score_dict,
                        ).0
                    }
                };

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
                    println!("{} {} {} {}", x+1, y+1, score, depth); // TODO: remove this debug print
                }
            }
        }
        
        if depth == self.max_depth {
            println!("Board score : {}", self.evaluation.score(board)); // TODO: remove this debug print
            println!("Deep score : {}", value); // TODO: remove this debug print
            println!("Best move : {:?}", (best_move.0 + 1, best_move.1 + 1)); // TODO: remove this debug print
        }

        (value, Some(best_move))
    }
}
