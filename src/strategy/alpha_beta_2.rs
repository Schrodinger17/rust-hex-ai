use std::{rc::Rc, time::Duration};

use crate::{board::Board, color::Color, evaluation::Evaluation, score::Score};

use super::Strategy;

#[derive(Clone)]
pub struct AlphaBeta2 {
    duration: Option<Duration>,
    max_depth: usize,
    evaluation: Rc<dyn Evaluation>,
}

impl Strategy for AlphaBeta2 {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.alpha_beta(board, color, self.max_depth, self.duration),
            Some(duration) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.alpha_beta(board, color, depth, self.duration);
                while time.elapsed() < duration && depth < self.max_depth {
                    depth += 1;
                    best_move = self.alpha_beta(board, color, depth, self.duration);
                }
                println!(
                    "Depth: {} in {:?}",
                    depth,
                    Duration::from_millis(time.elapsed().as_millis() as u64)
                );
                best_move
            }
        }
    }
}

impl AlphaBeta2 {
    #[allow(dead_code)]
    pub fn new(
        evaluation: Rc<dyn Evaluation>,
        max_depth: usize,
        duration: Option<Duration>,
    ) -> AlphaBeta2 {
        AlphaBeta2 {
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

    fn possible_moves_sorted(&self, board: &Board, color: Color) -> Vec<(usize, usize)> {
        let mut s_moves = board
            .possible_moves()
            .iter()
            .map(|(x, y)| {
                let mut board = board.clone();
                board.set(*x, *y, color);
                let score = self.evaluation.score(&board);
                //println!("{} {} {}", x+1, y+1, score);
                ((*x, *y), score)
            })
            .collect::<Vec<((usize, usize), Score)>>();

        s_moves.sort_by(|(_, score_a), (_, score_b)| score_a.partial_cmp(score_b).unwrap());

        if let Color::White = color {
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
        let nb_max_moves = board.size() * board.size();
        if possible_moves.len() > nb_max_moves {
            return possible_moves[0..nb_max_moves].to_vec();
        }
        possible_moves
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
            (_score, Some((x, y))) => {
                //println!("Score: {} with depth {}", score, depth);
                (x, y)
            }
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
        let mut possible_moves = self.possible_moves_sorted(board, color);

        possible_moves = self.keep_bests_moves(board, possible_moves);

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
    use crate::*;

    use super::*;

    #[ignore]
    #[test]
    fn test_mini_max() {
        let minimax = AlphaBeta2::new(Rc::new(Evaluation1::new()), 5, None);
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
