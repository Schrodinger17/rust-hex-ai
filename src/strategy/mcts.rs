use std::{rc::Rc, rc::Rc, time::Duration};

use rand::seq::SliceRandom;
use rand::Rng;

use crate::{board::Board, color::Color, evaluation::Evaluation, score::Score};

use super::Strategy;

#[derive(Clone)]
pub struct MCTS {
    duration: Option<Duration>,
    max_games: usize,
    evaluation: Rc<dyn Evaluation>,
}

impl Strategy for MCTS {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        // update duration if it's not None
        match duration {
            None => board.a_possible_move(),
            Some(duration) => {
                let time = std::time::Instant::now();

                let root = Node::new(None, color, None);
                let mut games_played = 0;
                while time.elapsed() < duration && games_played < self.max_games {
                    // MCTS Main steps
                    let (node, state) = self.select_node(&root, &board);
                    let outcome = self.roll_out(&state, color);
                    self.back_propagate(&node, outcome);

                    games_played += 1;
                }

                let best_move = root.best_child().move_played;

                println!(
                    "Games Played : {} in {:?}",
                    games_played,
                    Duration::from_millis(time.elapsed().as_millis() as u64)
                );

                return best_move.unwrap();
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    move_played: Option<(usize, usize)>,
    color: Color,
    parent: Option<Rc<Node>>,
    pub children: Vec<Rc<Node>>,
    n: f64,
    q: f64,
}

impl Node {
    fn new(move_played: Option<(usize, usize)>, color: Color, parent: Option<Rc<Node>>) -> Node {
        Node {
            move_played,
            color,
            parent,
            children: Vec::new(),
            n: 0.0,
            q: 0.0,
        }
    }

    fn create_children(&mut self, board: &Board) {
        board.possible_moves().iter().for_each(|(x, y)| {
            self.children.push(Rc::new(Node::new(
                Some((*x, *y)),
                self.color.opponent(),
                Some(Rc::new(self.clone())),
            )));
        });
    }

    fn value(&self) -> f64 {
        if self.n == 0.0 {
            return f64::MAX;
        } else {
            return self.q / self.n
                + (2.0 as f64).sqrt() * (self.parent.as_deref().unwrap().n.ln() / self.n).sqrt();
        }
    }

    fn best_child(&self) -> Rc<Node> {
        let best_value = self
            .children
            .iter()
            .map(|child| child.value())
            .into_iter()
            .reduce(f64::max)
            .unwrap();

        let best_children: Vec<Rc<Node>> = self
            .children
            .iter()
            .filter(|child| child.value() == best_value)
            .map(|child| child.clone())
            .collect();

        return best_children
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
    }
}

impl MCTS {
    #[allow(dead_code)]
    pub fn new(
        evaluation: Rc<dyn Evaluation>,
        max_games: usize,
        duration: Option<Duration>,
    ) -> MCTS {
        MCTS {
            evaluation,
            max_games,
            duration,
        }
    }

    #[allow(dead_code)]
    pub fn set_duration(&mut self, duration: Option<Duration>) {
        self.duration = duration;
    }

    #[allow(dead_code)]
    pub fn set_max_games(&mut self, max_games: usize) {
        self.max_games = max_games;
    }

    fn select_node(&self, node: &Node, board: &Board) -> (Node, Board) {
        let mut node = node;
        let mut state = board.clone();

        while !node.children.is_empty() {
            node = &node.best_child();

            let (x, y) = node.move_played.unwrap();
            state.set(x, y, node.color);

            if node.n == 0.0 {
                return (*node, state);
            }
        }

        if self.expand(node, &mut state) {
            node = node.children.choose(&mut rand::thread_rng()).unwrap();

            let (x, y) = node.move_played.unwrap();
            state.set(x, y, node.color);
        }

        return (*node, state);
    }

    fn expand(&self, node: &Node, state: &mut Board) -> bool {
        if state.is_finished() {
            return false;
        }

        node.create_children(state);
        return true;
    }

    fn roll_out(&self, state: &Board, mut color: Color) -> Score {
        let mut state = state.clone();
        let mut rng = rand::thread_rng();

        while !state.is_finished() {
            let (x, y) = state.possible_moves().choose(&mut rng).unwrap();
            state.set(*x, *y, color);
            color = color.opponent();
        }

        return state.winner().unwrap().win_score();
    }

    fn back_propagate(&self, node: &Node, outcome: Score) {
        let mut reward = match outcome {
            Score::WhiteCheckMate => 1.0,
            Score::BlackCheckMate => -1.0,
            _ => 0.0,
        };

        let mut node = node;

        while node.parent.is_some() {
            node.n += 1.0;
            node.q += reward;

            node = node.parent.unwrap().as_ref();

            reward = -reward;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluation::evaluation1::Evaluation1;

    use super::*;

    #[ignore]
    #[test]
    fn test_mcts() {
        let mcts = MCTS::new(Rc::new(Evaluation1::new()), 7, None);
        let mut board = Board::new(4);
        //board.set(0, 0, Color::White);
        //board.set(1, 0, Color::White);
        //board.set(1, 1, Color::White);
        board.set(1, 1, Color::Black);

        println!("{}", board);
        let best_move = mcts.next_move(&board, Color::White, None);
        println!("{:?}", best_move);
    }
}
