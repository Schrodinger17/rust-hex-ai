use std::{rc::Rc, time::Duration};

use crate::{
    board::Board,
    color::Color,
    evaluation::Evaluation,
    log::{self, LogFlag, LogLevel},
    score::Score,
};

use super::Strategy;

#[derive(Clone)]
pub struct AlphaBeta {
    max_depth: usize,
    evaluation: Rc<dyn Evaluation>,
    log_level: Rc<LogLevel>,
}

impl Strategy for AlphaBeta {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.alpha_beta(board, color, self.max_depth, duration),
            Some(duration_unwrapped) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.alpha_beta(board, color, depth, duration);
                while time.elapsed() < duration_unwrapped && depth < self.max_depth {
                    depth += 1;
                    best_move = self.alpha_beta(board, color, depth, duration);
                }
                if self.log_level.is(LogFlag::SearchDepth) {
                    println!(
                        "Depth: {} in {:?}",
                        depth,
                        Duration::from_millis(time.elapsed().as_millis() as u64)
                    );
                }
                best_move
            }
        }
    }
}

impl AlphaBeta {
    #[allow(unused)]
    pub fn new(
        evaluation: Rc<dyn Evaluation>,
        max_depth: usize,
        log_level: Rc<LogLevel>,
    ) -> AlphaBeta {
        AlphaBeta {
            evaluation,
            max_depth,
            log_level,
        }
    }

    #[allow(unused)]
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
        match self._alpha_beta(
            board,
            color,
            depth,
            Score::BlackCheckMate,
            Score::WhiteCheckMate,
            duration,
        ) {
            (_, Some((x, y))) => (x, y),
            _ => panic!("Error in alpha_beta"),
        }
    }

    fn _alpha_beta(
        &self,
        board: &Board,
        color: Color,
        depth: usize,
        alpha: Score,
        beta: Score,
        duration: Option<Duration>,
    ) -> (Score, Option<(usize, usize)>) {
        let mut alpha = alpha;
        let mut beta = beta;

        let mut best_moves: Vec<(Score, (usize, usize))> = Vec::new();

        if board.is_win(color) {
            return (color.win_score(), None);
        } else if board.is_win(color.opponent()) {
            return (color.opponent().win_score(), None);
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
        let possible_moves = board.possible_moves();

        if color == Color::White {
            value = Score::BlackCheckMate;
            for (x, y) in possible_moves {
                let mut new_board = board.clone();
                new_board.set(x, y, color);

                let (score, _) = self._alpha_beta(
                    &new_board,
                    color.opponent(),
                    depth - 1,
                    alpha,
                    beta,
                    duration,
                );

                best_moves.push((score, (x, y)));

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
                new_board.set(x, y, color);
                let (score, _) = self._alpha_beta(
                    &new_board,
                    color.opponent(),
                    depth - 1,
                    alpha,
                    beta,
                    duration,
                );

                best_moves.push((score, (x, y)));

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

        let move_cmp = |a: &(Score, _), b: &(Score, _)| {
            if color == Color::White {
                PartialOrd::partial_cmp(&b.0, &a.0).unwrap()
                //b.0.partial_cmp(&a.0).unwrap()
            } else {
                PartialOrd::partial_cmp(&a.0, &b.0).unwrap()
                //a.0.partial_cmp(&b.0).unwrap()
            }
        };

        if depth == self.max_depth {
            println!("{}", board);
            best_moves.sort_by(move_cmp);
            println!("{:?}", best_moves);
            println!("Board score : {}", self.evaluation.score(board)); // TODO: remove this debug print
            println!("Deep score : {}", value); // TODO: remove this debug print
            println!("Best move : {:?}", (best_move.0 + 1, best_move.1 + 1)); // TODO: remove this debug print
        }

        (value.next_back().unwrap(), Some(best_move))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[ignore]
    #[test]
    fn test_mini_max() {
        let minimax = AlphaBeta::new(Rc::new(Evaluation1::new()), 7, Rc::default());
        let mut board = Board::new(4);
        //board.set(0, 0, Color::White);
        //board.set(1, 0, Color::White);
        //board.set(1, 1, Color::White);
        board.set(1, 1, Color::Black);

        println!("{}", board);
        let best_move = minimax.next_move(&board, Color::White, None);
        println!("{:?}", best_move);
    }
}
