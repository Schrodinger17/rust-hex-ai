
trait Strategy {
    fn play(&mut self, board: &Board, color: Color) -> Option<Position>;
}

trait Evaluation {
    fn score(&self, board: &Board) -> f64;
}


struct Evaluation1;

impl Evaluation for Evaluation1 {
    fn score(&self, board: &Board) -> f64 {
        0.0
    }
}

pub struct MiniMax {
    duration: Option<Duration>,
    max_depth: usize,
    evaluation: Box<dyn Evaluation>,
}

impl MiniMax {
    fn new(evaluation: Box<dyn Evaluation>, max_depth: usize, duration: Option<Duration>) -> MiniMax {
        MiniMax {
            evaluation,
            max_depth,
            duration,
        }
    }
}

impl Strategy for MiniMax {
    fn play(&mut self, board: &Board, color: Color) -> Option<Position> {
        self.evaluation.score(board);
        Some((0, 0))
    }
}