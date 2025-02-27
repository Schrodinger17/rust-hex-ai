#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Reachable(usize),
    Unreachable,
    Unexplored,
}

impl From<Distance> for Option<usize> {
    fn from(distance: Distance) -> Self {
        match distance {
            Distance::Reachable(value) => Some(value),
            Distance::Unreachable | Distance::Unexplored => None,
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Distance::Unexplored, Distance::Unexplored) => Ordering::Equal,
            (Distance::Unreachable, Distance::Unreachable) => Ordering::Equal,

            (Distance::Reachable(a), Distance::Reachable(b)) => a.cmp(b),

            (Distance::Reachable(_), _) => Ordering::Less,
            (_, Distance::Reachable(_)) => Ordering::Greater,

            (Distance::Unreachable, Distance::Unexplored) => Ordering::Greater,
            (Distance::Unexplored, Distance::Unreachable) => Ordering::Less,
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
            Distance::Reachable(1).cmp(&Distance::Reachable(1)),
            Ordering::Equal
        );
        assert_eq!(
            Distance::Unreachable.cmp(&Distance::Unreachable),
            Ordering::Equal
        );
        assert_eq!(
            Distance::Unexplored.cmp(&Distance::Unexplored),
            Ordering::Equal
        );

        // Less
        assert_eq!(
            Distance::Reachable(1).cmp(&Distance::Reachable(2)),
            Ordering::Less
        );
        assert_eq!(
            Distance::Unexplored.cmp(&Distance::Unreachable),
            Ordering::Less
        );

        // Greater
        assert_eq!(
            Distance::Reachable(2).cmp(&Distance::Reachable(1)),
            Ordering::Greater
        );
        assert_eq!(
            Distance::Unreachable.cmp(&Distance::Reachable(1)),
            Ordering::Greater
        );
        assert_eq!(
            Distance::Unexplored.cmp(&Distance::Reachable(1)),
            Ordering::Greater
        );

        let distances = vec![
            Distance::Reachable(1),
            Distance::Unexplored,
            Distance::Unreachable,
            Distance::Reachable(0),
        ];
        assert_eq!(distances.iter().min(), Some(&Distance::Reachable(0)));
    }
}
