use std::collections::HashMap;
use std::f64;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::board::Board;
use crate::color::Color;
use crate::evaluation::Evaluation;
use crate::log::LogLevel;
use crate::score::Score;

use super::Strategy;

#[derive(Debug, Clone)]
struct MCTSNode {
    visits: usize,
    wins: f64,
    children: HashMap<(usize, usize), MCTSNode>,
}

impl MCTSNode {
    pub fn new() -> Self {
        MCTSNode {
            visits: 0,
            wins: 0.0,
            children: HashMap::new(),
        }
    }

    /// UCB1 Formula: Exploitation + Exploration
    fn ucb1(&self, parent_visits: usize) -> f64 {
        if self.visits == 0 {
            f64::INFINITY
        } else {
            (self.wins / self.visits as f64)
                + 1.41 * ((parent_visits as f64).ln() / self.visits as f64).sqrt()
        }
    }
}

#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
pub struct MCTS {
    evaluation: Rc<dyn Evaluation>,
    log_level: Rc<LogLevel>,
}

impl Strategy for MCTS {
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        let time_limit = duration.unwrap_or(Duration::from_secs(1)); // Default 1s if not provided
        let mut board_clone = board.clone();
        self.mcts_search(&mut board_clone, color, time_limit)
    }
}

impl MCTS {
    #[allow(unused)]
    pub fn new(evaluation: Rc<dyn Evaluation>, log_level: Rc<LogLevel>) -> Self {
        MCTS {
            evaluation,
            log_level,
        }
    }

    /// Run MCTS with time constraint
    fn mcts_search(&self, board: &mut Board, color: Color, duration: Duration) -> (usize, usize) {
        let mut root = MCTSNode::new();
        let start_time = Instant::now();

        while start_time.elapsed() < duration {
            let mut temp_board = board.clone();
            self.simulate_mcts(&mut root, &mut temp_board, color);
        }

        // Choose the move with the most visits
        root.children
            .iter()
            .max_by_key(|(_, node)| node.visits)
            .map(|(mv, _)| *mv)
            .unwrap()
    }
    fn simulate_mcts(&self, node: &mut MCTSNode, board: &mut Board, color: Color) -> Score {
        // 🔹 Check for terminal state (no moves left)
        let possible_moves = board.possible_moves();
        if possible_moves.is_empty() {
            return self.evaluation.score(board); // Return final board score
        }

        let mut best_move = None;
        let mut best_ucb1 = f64::NEG_INFINITY;

        // Selection: Pick best UCB1 move
        for (mv, child) in &node.children {
            let ucb1 = child.ucb1(node.visits);
            if ucb1 > best_ucb1 {
                best_ucb1 = ucb1;
                best_move = Some(*mv);
            }
        }

        // Expansion: Pick an unexplored move
        if best_move.is_none() {
            best_move = Some(possible_moves[0]); // Safe because we checked empty case above
        }

        if let Some((x, y)) = best_move {
            let next_color = color.opponent(); // 🔹 Switch player for simulation!

            // Play the move
            board.set(x, y, color);

            // Add new child if not present
            let child_node = node.children.entry((x, y)).or_insert_with(MCTSNode::new);

            // 🔹 Recursive Simulation (Alternate Players!)
            let score = -self.simulate_mcts(child_node, board, next_color); // 🔄 Switch turns!

            // Backpropagation
            child_node.visits += 1;
            child_node.wins += &score.into();

            score
        } else {
            Score::Advantage(0.0)
        }
    }
}
