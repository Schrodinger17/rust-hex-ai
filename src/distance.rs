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
}
