#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Reachable(usize),
    Unreachable,
    Unexplored,
}

impl Distance {
    #[allow(unused)]
    pub fn unwrap(self) -> usize {
        match self {
            Distance::Reachable(d) => d,
            Distance::Unreachable => panic!("Can't unwrap Distance::Unreachable"),
            Distance::Unexplored => panic!("Can't unwrap Distance::Unexplored"),
        }
    }
}

use std::cmp::Ordering;

impl std::cmp::PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Distance::Unexplored, _) => None,
            (_, Distance::Unexplored) => None,
            (Distance::Reachable(a), Distance::Reachable(b)) => a.partial_cmp(b),
            (_, Distance::Reachable(_)) => Some(Ordering::Greater),
            (Distance::Reachable(_), _) => Some(Ordering::Less),
            (Distance::Unreachable, Distance::Unreachable) => Some(Ordering::Equal),
        }
    }
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

#[allow(dead_code, unused_variables)]
fn print_distances(distances: Vec<Vec<Distance>>) {
    /*
    let board_size = distances.len();
    let mut result = String::new();
    let mut formatter = Formatter::new(&mut result);

    write_column_labels(&mut formatter, board_size, 0).unwrap();

    for row in 0..board_size {
        write_row(&mut formatter, &Board::new(board_size), row).unwrap();
        write_indent(&mut formatter, row).unwrap();
        write!(&mut formatter, " ").unwrap();
        for column in 0..board_size {
            write!(&mut formatter, "{} ", distances[row][column]).unwrap();
        }
        writeln!(&mut formatter).unwrap();
    }

    write_row(&mut formatter, &Board::new(board_size), board_size).unwrap();
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_distance_vec() {
        let distances = vec![
            vec![
                Distance::Reachable(0),
                Distance::Reachable(1),
                Distance::Reachable(2),
            ],
            vec![
                Distance::Reachable(1),
                Distance::Unreachable,
                Distance::Unreachable,
            ],
            vec![
                Distance::Reachable(2),
                Distance::Unreachable,
                Distance::Unreachable,
            ],
        ];

        print_distances(distances);
    }

    #[test]
    fn cmp() {
        // Equal
        assert_eq!(
            Distance::Reachable(1).partial_cmp(&Distance::Reachable(1)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Distance::Unreachable.partial_cmp(&Distance::Unreachable),
            Some(Ordering::Equal)
        );

        // Less
        assert_eq!(
            Distance::Reachable(1).partial_cmp(&Distance::Reachable(2)),
            Some(Ordering::Less)
        );

        // Greater
        assert_eq!(
            Distance::Reachable(2).partial_cmp(&Distance::Reachable(1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Distance::Unreachable.partial_cmp(&Distance::Reachable(1)),
            Some(Ordering::Greater)
        );

        // Incomparable
        assert_eq!(
            Distance::Unexplored.partial_cmp(&Distance::Unexplored),
            None
        );
        assert_eq!(
            Distance::Unexplored.partial_cmp(&Distance::Unreachable),
            None
        );
        assert_eq!(
            Distance::Unexplored.partial_cmp(&Distance::Reachable(1)),
            None
        );
    }
}
