use std::{rc::Rc, time::Duration};

use crate::{
    board::Board,
    color::Color,
    evaluation::Evaluation,
    log::{LogFlag, LogLevel},
    score::Score,
};

use super::Strategy;

#[derive(Clone)]
pub struct AlphaBeta4 {
    max_depth: usize,
    evaluation: Rc<dyn Evaluation>,
    log_level: Rc<LogLevel>,
}

impl Strategy for AlphaBeta4 {
    fn next_move(&self, board: &Board, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.alpha_beta(board, self.max_depth, duration),
            Some(duration_unwrapped) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.alpha_beta(board, depth, duration);
                while time.elapsed() < duration_unwrapped && depth < self.max_depth {
                    depth += 1;
                    best_move = self.alpha_beta(board, depth, duration);
                }
                if self.log_level.is(LogFlag::SearchDepth) {
                    println!("Depth: {} in {:?}", depth, time.elapsed());
                }
                best_move
            }
        }
    }
}

impl AlphaBeta4 {
    #[allow(unused)]
    pub fn new(
        evaluation: Rc<dyn Evaluation>,
        max_depth: usize,
        log_level: Rc<LogLevel>,
    ) -> AlphaBeta4 {
        AlphaBeta4 {
            evaluation,
            max_depth,
            log_level,
        }
    }

    #[allow(unused)]
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }

    fn possible_moves_sorted(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut s_moves = board
            .possible_moves()
            .iter()
            .map(|(x, y)| {
                let mut board = board.clone();
                board.play(*x, *y);
                let score = self.evaluation.score(&board);
                //println!("{} {} {}", x+1, y+1, score);
                ((*x, *y), score)
            })
            .collect::<Vec<((usize, usize), Score)>>();

        s_moves.sort_by(|(_, score_a), (_, score_b)| score_a.partial_cmp(score_b).unwrap());

        if let Color::White = board.next_color() {
            s_moves.reverse();
        }

        s_moves
            .iter()
            .map(|((x, y), _)| (*x, *y))
            .collect::<Vec<(usize, usize)>>()
    }

    fn keep_bests_moves(
        &self,
        board: &Board,
        possible_moves: Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let nb_max_moves = board.size() * board.size() / 4;
        if possible_moves.len() > nb_max_moves {
            return possible_moves[0..nb_max_moves].to_vec();
        }
        possible_moves
    }

    fn alpha_beta(
        &self,
        board: &Board,
        depth: usize,
        duration: Option<Duration>,
    ) -> (usize, usize) {
        match self._alpha_beta(
            board,
            depth,
            Score::BlackCheckMate,
            Score::WhiteCheckMate,
            duration,
        ) {
            (score, Some((x, y))) => {
                if self.log_level.is(LogFlag::Score) {
                    println!("Score: {} with depth {}", score, depth);
                }
                (x, y)
            }
            _ => panic!("Error in alpha_beta"),
        }
    }

    fn _alpha_beta(
        &self,
        board: &Board,
        depth: usize,
        alpha: Score,
        beta: Score,
        duration: Option<Duration>,
    ) -> (Score, Option<(usize, usize)>) {
        let mut alpha = alpha;
        let mut beta = beta;

        if let Some(winner) = board.winner() {
            return (winner.win_score(), None);
        }

        if depth == 0 {
            return (self.evaluation.score(board), None);
        }

        if let Some(duration) = duration {
            if duration.as_millis() < 100 {
                return (self.evaluation.score(board), None);
            }
        }

        let mut value: Score;
        let mut best_move = board.first_possible_move().unwrap();
        let mut possible_moves = self.possible_moves_sorted(board);

        possible_moves = self.keep_bests_moves(board, possible_moves);

        if board.next_color() == Color::White {
            value = Score::BlackCheckMate;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.play(x, y);

                let (score, _) = self._alpha_beta(&new_board, depth - 1, alpha, beta, duration);

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
            value = Score::WhiteCheckMate;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.play(x, y);
                let (score, _) = self._alpha_beta(&new_board, depth - 1, alpha, beta, duration);
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

        if depth == self.max_depth {
            //println!("Board score : {}", self.evaluation.score(board)); // TODO: remove this debug print
            //println!("Deep score : {}", value); // TODO: remove this debug print
            //println!("Best move : {:?}", (best_move.0 + 1, best_move.1 + 1)); // TODO: remove this debug print
        }

        (value.next_back().unwrap(), Some(best_move))
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluation::Evaluation1;

    use super::*;

    #[ignore]
    #[test]
    fn alpha_beta_4() {
        let player = AlphaBeta4::new(Rc::new(Evaluation1::new()), 5, Rc::default());
        let mut board = Board::new();
        board.play(0, 0);
        board.play(1, 0);
        board.play(1, 1);
        board.play(1, 1);

        println!("{}", board);
        let best_move = player.next_move(&board, None);
        println!("{:?}", best_move);
    }
}
