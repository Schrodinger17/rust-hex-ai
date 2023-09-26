use core::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub enum Score {
    Advantage(f64),
    BlackMateIn(usize),
    WhiteMateIn(usize),
    BlackCheckMate,
    WhiteCheckMate,
}

impl Score {
    pub fn to_f64(&self) -> f64 {
        let max: f64 = 1000.0;//f64::MAX;
        let min: f64 = -1000.0;//f64::MIN;

        match self {
            Score::Advantage(score) => *score,
            Score::BlackMateIn(n) => min + *n as f64,
            Score::WhiteMateIn(n) => max - *n as f64,
            Score::BlackCheckMate => min,
            Score::WhiteCheckMate => max,
        }
    }

    pub fn from_f64(score: f64) -> Score {
        let max: f64 = 1000.0;//f64::MAX;
        let min: f64 = -1000.0;//f64::MIN;
        let treshold: f64 = 50.0;

        match score{
            score if score >= max => Score::WhiteCheckMate,
            score if score <= min => Score::BlackCheckMate,
            score if score > max - treshold => Score::WhiteMateIn((max - score) as usize),
            score if score < min + treshold => Score::BlackMateIn((score - min) as usize),
            score if score > 0.0 => Score::Advantage(score),
            score if score < 0.0 => Score::Advantage(score),
            score if score == 0.0 => Score::Advantage(score),
            _ => panic!("Error in Score::from_f64"),
        }
    }

    pub fn previous(&self) -> Score {
        match self {
            Score::Advantage(score) => Score::Advantage(*score),
            Score::BlackMateIn(n) => Score::BlackMateIn(n + 1),
            Score::WhiteMateIn(n) => Score::WhiteMateIn(n + 1),
            Score::BlackCheckMate => Score::BlackMateIn(1),
            Score::WhiteCheckMate => Score::WhiteMateIn(1),
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Score::Advantage(score) => write!(f, "{:.2}", score),
            Score::BlackMateIn(moves) => write!(f, "Black mate in {}", moves),
            Score::WhiteMateIn(moves) => write!(f, "White mate in {}", moves),
            Score::BlackCheckMate => write!(f, "Black checkmate"),
            Score::WhiteCheckMate => write!(f, "White checkmate"),
        }
    }
}

impl PartialEq<Score> for Score {
    fn eq(&self, other: &Score) -> bool {
        self.to_f64() == other.to_f64()
    }
}

impl PartialOrd<Score> for Score {
    fn partial_cmp(&self, other: &Score) -> Option<std::cmp::Ordering> {
        self.to_f64().partial_cmp(&other.to_f64())
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        match (self, other) {
            (Score::Advantage(score1), Score::Advantage(score2)) => Score::Advantage(score1 + score2),
            
            (Score::WhiteCheckMate, _) => Score::WhiteCheckMate,
            (_, Score::WhiteCheckMate) => Score::WhiteCheckMate,
            (Score::BlackCheckMate, _) => Score::BlackCheckMate,
            (_, Score::BlackCheckMate) => Score::BlackCheckMate,

            (Score::BlackMateIn(n), _) => Score::WhiteMateIn(n),
            (_, Score::BlackMateIn(n)) => Score::WhiteMateIn(n),
            (Score::WhiteMateIn(n), _) => Score::BlackMateIn(n),
            (_, Score::WhiteMateIn(n)) => Score::BlackMateIn(n),
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
        assert_eq!(score, Score::WhiteMateIn(1));
    }

    #[test]
    fn previous_score() {
        let score = Score::BlackCheckMate.previous();
        assert_eq!(score, Score::BlackMateIn(1));

        let score = Score::WhiteCheckMate.previous();
        assert_eq!(score, Score::WhiteMateIn(1));

        let score = Score::BlackMateIn(1).previous();
        assert_eq!(score, Score::BlackMateIn(2));

        let score = Score::WhiteMateIn(1).previous();
        assert_eq!(score, Score::WhiteMateIn(2));

        let score = Score::Advantage(1.0).previous();
        assert_eq!(score, Score::Advantage(1.0));
    }
}