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
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => self.minimax(board, color, self.max_depth, duration),
            Some(duration_unwrapped) => {
                let time = std::time::Instant::now();
                let mut depth = 1;
                let mut best_move = self.minimax(board, color, depth, duration);
                while time.elapsed() < duration_unwrapped && depth < self.max_depth {
                    depth += 1;
                    best_move = self.minimax(board, color, depth, duration);
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

    fn minimax(
        &self,
        board: &Board,
        color: Color,
        depth: usize,
        duration: Option<Duration>,
    ) -> (usize, usize) {
        match self._minimax(board, color, depth, duration) {
            (_, Some((x, y))) => (x, y),
            _ => panic!("Error in minimax"),
        }
    }

    fn _minimax(
        &self,
        board: &Board,
        color: Color,
        depth: usize,
        duration: Option<Duration>,
    ) -> (Score, Option<(usize, usize)>) {
        if board.is_win(color.opponent()) {
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

        let mut best_score = color.opponent().win_score();
        let mut best_move = board.first_possible_move().unwrap();

        let mut best_moves: Vec<(Score, (usize, usize))> = Vec::new();

        for (x, y) in board.possible_moves() {
            let mut new_board = board.clone();
            new_board.set(x, y, color);
            let (score, _) = self._minimax(&new_board, color.opponent(), depth - 1, duration);
            if (color == Color::White && score > best_score) || score < best_score {
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
    fn test_mini_max() {
        let minimax = MiniMax::new(Rc::new(Evaluation1::new()), 9, Rc::default());
        let mut board = Board::new();
        //board.set(0, 0, Color::White);
        //board.set(1, 0, Color::White);
        //board.set(1, 1, Color::White);
        board.set(1, 1, Color::Black);

        println!("{}", board);
        let best_move = minimax.next_move(&board, Color::White, None);
        println!("{:?}", best_move);
    }
}
