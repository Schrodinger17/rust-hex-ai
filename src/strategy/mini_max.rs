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
pub struct MiniMax {
    max_depth: usize,
    evaluation: Rc<dyn Evaluation>,
    log_level: Rc<LogLevel>,
}

impl Strategy for MiniMax {
    fn next_move(&self, board: &Board, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.minimax(board, self.max_depth, duration),
            Some(duration_unwrapped) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.minimax(board, depth, duration);
                while time.elapsed() < duration_unwrapped && depth < self.max_depth {
                    depth += 1;
                    best_move = self.minimax(board, depth, duration);
                }
                if self.log_level.is(LogFlag::SearchDepth) {
                    println!("Depth: {} in {:?}", depth, time.elapsed());
                }
                best_move
            }
        }
    }
}

impl MiniMax {
    #[allow(unused)]
    pub fn new(
        evaluation: Rc<dyn Evaluation>,
        max_depth: usize,
        log_level: Rc<LogLevel>,
    ) -> MiniMax {
        MiniMax {
            evaluation,
            max_depth,
            log_level: Rc::default(),
        }
    }

    #[allow(unused)]
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }

    fn minimax(&self, board: &Board, depth: usize, duration: Option<Duration>) -> (usize, usize) {
        match self._minimax(board, depth, duration) {
            (score, Some((x, y))) => {
                if self.log_level.is(LogFlag::Score) {
                    println!("Score: {} with depth {}", score, depth);
                }
                (x, y)
            }
            _ => panic!("Error in minimax"),
        }
    }

    fn _minimax(
        &self,
        board: &Board,
        depth: usize,
        duration: Option<Duration>,
    ) -> (Score, Option<(usize, usize)>) {
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

        let mut best_score = board.next_color().opponent().win_score();
        let mut best_move = board.first_possible_move().unwrap();

        let mut best_moves: Vec<(Score, (usize, usize))> = Vec::new();

        for (x, y) in board.possible_moves() {
            let mut new_board = board.clone();
            new_board.play(x, y);
            let (score, _) = self._minimax(&new_board, depth - 1, duration);
            if (board.next_color() == Color::White && score > best_score) || score < best_score {
                best_score = score;
                best_move = (x, y);
            }

            best_moves.push((score, (x, y)));

            if depth == self.max_depth {
                //println!("{} {} {}", x+1, y+1, score); // TODO: remove this debug print
            }
        }

        /*
        let move_cmp = |a: &(Score, (usize, usize)), b: &(Score, (usize, usize))| {
            if color == Color::White {
                b.0.partial_cmp(&a.0).unwrap()
            } else {
                a.0.partial_cmp(&b.0).unwrap()
            }
        };

        if depth == self.max_depth {
            println!("{}", board);
            best_moves.sort_by( move_cmp );
            println!("{:?}", best_moves);
            println!("Board score : {}", self.evaluation.score(board)); // TODO: remove this debug print
            println!("Deep score : {}", best_score); // TODO: remove this debug print
            println!("Best move : {:?}", (best_move.0 + 1, best_move.1 + 1)); // TODO: remove this debug print
        }*/

        (best_score.next_back().unwrap(), Some(best_move))
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluation::Evaluation1;

    use super::*;

    #[ignore]
    #[test]
    fn mini_max() {
        let player = MiniMax::new(Rc::new(Evaluation1::new()), 5, Rc::default());
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
