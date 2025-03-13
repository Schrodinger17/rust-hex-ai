use std::collections::HashSet;

#[allow(unused)]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum LogFlag {
    SearchDepth,
    Score,
    Moves,
    Position,
    GameResult,
    MatchResult,
}

#[derive(Default, Clone, Debug)]
pub struct LogLevel {
    flags: HashSet<LogFlag>,
}

impl LogLevel {
    #[allow(unused)]
    pub fn new() -> LogLevel {
        LogLevel::default()
    }

    #[allow(unused)]
    pub fn add(&mut self, flag: LogFlag) -> &mut Self {
        self.flags.insert(flag);
        self
    }

    #[allow(unused)]
    pub fn remove(&mut self, flag: LogFlag) -> &mut Self {
        self.flags.remove(&flag);
        self
    }

    pub fn is(&self, flag: LogFlag) -> bool {
        self.flags.contains(&flag)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use super::LogLevel;

    #[test]
    fn log_level() {
        let mut log_level = LogLevel::new();

        log_level.add(LogFlag::Position).add(LogFlag::MatchResult);

        assert!(log_level.is(LogFlag::MatchResult));
        assert!(log_level.is(LogFlag::Position));

        log_level.remove(LogFlag::Position);
        assert!(!log_level.is(LogFlag::Position));
    }
}
