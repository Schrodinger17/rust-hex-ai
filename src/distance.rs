#[derive(Debug, Clone, Copy)]
pub enum Distance {
    Reachable(usize),
    Unreachable,
    Unexplored,
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distance::Reachable(d) => write!(f, "{}", d),
            Distance::Unreachable => write!(f, "X"),
            Distance::Unexplored => write!(f, "."),
        }
    }
}