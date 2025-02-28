use core::fmt;
use std::{cmp::min, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Score {
    #[allow(unused)]
    Undefined,
    Advantage(f64),
    BlackMateIn(usize),
    WhiteMateIn(usize),
    BlackCheckMate,
    WhiteCheckMate,
}

impl Score {
    pub const MIN: f64 = -1000.0; //f64::MIN;
    pub const MAX: f64 = 1000.0; //f64::MAX;
}

impl Iterator for Score {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if *self == Score::WhiteCheckMate || *self == Score::BlackCheckMate {
            return None;
        }

        *self = match self {
            Score::Undefined => Score::Undefined,
            Score::Advantage(score) => Score::Advantage(*score),
            Score::BlackMateIn(n) if *n == 1 => Score::BlackCheckMate,
            Score::WhiteMateIn(n) if *n == 1 => Score::WhiteCheckMate,
            Score::BlackMateIn(n) => Score::BlackMateIn(*n - 1),
            Score::WhiteMateIn(n) => Score::WhiteMateIn(*n - 1),
            _ => unreachable!(),
        };

        Some(*self)
    }
}

impl DoubleEndedIterator for Score {
    fn next_back(&mut self) -> Option<Self::Item> {
        *self = match self {
            Score::Undefined => Score::Undefined,
            Score::Advantage(score) => Score::Advantage(*score),
            Score::BlackMateIn(n) => Score::BlackMateIn(*n + 1),
            Score::WhiteMateIn(n) => Score::WhiteMateIn(*n + 1),
            Score::BlackCheckMate => Score::BlackMateIn(1),
            Score::WhiteCheckMate => Score::WhiteMateIn(1),
        };

        Some(*self)
    }
}

impl From<Score> for f64 {
    fn from(score: Score) -> f64 {
        match score {
            Score::Undefined => 0.0,
            Score::Advantage(score) => score,
            Score::BlackMateIn(n) => Score::MIN + n as f64,
            Score::WhiteMateIn(n) => Score::MAX - n as f64,
            Score::BlackCheckMate => Score::MIN,
            Score::WhiteCheckMate => Score::MAX,
        }
    }
}

impl From<f64> for Score {
    fn from(score: f64) -> Score {
        let max: f64 = 1000.0; //f64::MAX;
        let min: f64 = -1000.0; //f64::MIN;
        let threshold: f64 = 50.0;

        match score {
            score if score >= max => Score::WhiteCheckMate,
            score if score <= min => Score::BlackCheckMate,
            score if score > max - threshold => Score::WhiteMateIn((max - score) as usize),
            score if score < min + threshold => Score::BlackMateIn((score - min) as usize),
            score if score > 0.0 => Score::Advantage(score),
            score if score < 0.0 => Score::Advantage(score),
            score if score == 0.0 => Score::Advantage(score),
            _ => panic!("Error in Score::from_f64"),
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Score::Undefined => write!(f, "Undefined"),
            Score::Advantage(score) => write!(f, "{:.2}", score),
            Score::BlackMateIn(moves) => write!(f, "Black mate in {}", moves),
            Score::WhiteMateIn(moves) => write!(f, "White mate in {}", moves),
            Score::BlackCheckMate => write!(f, "Black checkmate"),
            Score::WhiteCheckMate => write!(f, "White checkmate"),
        }
    }
}

impl PartialOrd<Score> for Score {
    fn partial_cmp(&self, other: &Score) -> Option<std::cmp::Ordering> {
        let left: f64 = (*self).into();
        let right: f64 = (*other).into();
        left.partial_cmp(&right)
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        match (self, other) {
            (Score::Advantage(score1), Score::Advantage(score2)) => {
                Score::Advantage(score1 + score2)
            }

            // Checkmate
            (Score::WhiteCheckMate, _) => Score::WhiteCheckMate,
            (_, Score::WhiteCheckMate) => Score::WhiteCheckMate,
            (Score::BlackCheckMate, _) => Score::BlackCheckMate,
            (_, Score::BlackCheckMate) => Score::BlackCheckMate,

            // Fastest mate
            (Score::BlackMateIn(n1), Score::BlackMateIn(n2)) => Score::BlackMateIn(min(n1, n2)),
            (Score::WhiteMateIn(n1), Score::WhiteMateIn(n2)) => Score::WhiteMateIn(min(n1, n2)),

            // Fist to mate
            (Score::BlackMateIn(n1), Score::WhiteMateIn(n2)) => {
                if n1 < n2 {
                    Score::BlackMateIn(n1)
                } else {
                    Score::WhiteMateIn(n2)
                }
            }
            (Score::WhiteMateIn(n1), Score::BlackMateIn(n2)) => {
                if n1 < n2 {
                    Score::WhiteMateIn(n1)
                } else {
                    Score::BlackMateIn(n2)
                }
            }

            // Forced mate
            (Score::BlackMateIn(n), _) => Score::BlackMateIn(n),
            (_, Score::BlackMateIn(n)) => Score::BlackMateIn(n),
            (Score::WhiteMateIn(n), _) => Score::WhiteMateIn(n),
            (_, Score::WhiteMateIn(n)) => Score::WhiteMateIn(n),

            // Undefined
            (Score::Undefined, s) => s,
            (s, Score::Undefined) => s,
        }
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order() {
        assert!(Score::Advantage(1.0) > Score::Advantage(0.0));
        assert!(Score::Advantage(0.0) < Score::Advantage(1.0));
        assert!(Score::Advantage(0.0) == Score::Advantage(0.0));

        assert!(Score::BlackMateIn(1) < Score::BlackMateIn(2));
        assert!(Score::BlackMateIn(2) > Score::BlackMateIn(1));
        assert!(Score::BlackMateIn(1) == Score::BlackMateIn(1));

        assert!(Score::WhiteMateIn(1) > Score::WhiteMateIn(2));
        assert!(Score::WhiteMateIn(2) < Score::WhiteMateIn(1));
        assert!(Score::WhiteMateIn(1) == Score::WhiteMateIn(1));

        assert!(Score::BlackCheckMate < Score::WhiteCheckMate);
        assert!(Score::WhiteCheckMate > Score::BlackCheckMate);
        assert!(Score::BlackCheckMate == Score::BlackCheckMate);
        assert!(Score::WhiteCheckMate == Score::WhiteCheckMate);

        assert!(Score::Advantage(0.0) > Score::BlackMateIn(1));
        assert!(Score::Advantage(0.0) < Score::WhiteMateIn(1));
        assert!(Score::Advantage(0.0) > Score::BlackCheckMate);
        assert!(Score::Advantage(0.0) < Score::WhiteCheckMate);
    }

    #[test]
    fn add_score() {
        let score = Score::Advantage(1.0) + Score::Advantage(1.0);
        assert_eq!(score, Score::Advantage(2.0));

        let score = Score::Advantage(1.0) + Score::Advantage(-1.0);
        assert_eq!(score, Score::Advantage(0.0));

        let score = Score::Advantage(1.0) + Score::BlackMateIn(1);
        assert_eq!(score, Score::BlackMateIn(1));

        let score = Score::WhiteMateIn(2) + Score::BlackMateIn(1);
        assert_eq!(score, Score::BlackMateIn(1));
    }

    #[test]
    fn iterator() {
        let score = Score::BlackCheckMate.next_back().unwrap();
        assert_eq!(score, Score::BlackMateIn(1));

        let score = Score::WhiteCheckMate.next_back().unwrap();
        assert_eq!(score, Score::WhiteMateIn(1));

        let score = Score::BlackMateIn(1).next_back().unwrap();
        assert_eq!(score, Score::BlackMateIn(2));

        let score = Score::WhiteMateIn(1).next_back().unwrap();
        assert_eq!(score, Score::WhiteMateIn(2));

        let score = Score::Advantage(1.0).next_back().unwrap();
        assert_eq!(score, Score::Advantage(1.0));

        let scores: Vec<_> = Score::BlackCheckMate.rev().take(3).collect();
        assert_eq!(
            scores,
            vec![
                Score::BlackMateIn(1),
                Score::BlackMateIn(2),
                Score::BlackMateIn(3)
            ]
        )
    }
}
