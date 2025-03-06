use core::fmt;
use std::collections::VecDeque;
use std::hash::Hash;

use rand::Rng;

use crate::cell::Cell;
use crate::color::Color;
use crate::distance::Distance;

//pub type Board = Board<11>;

pub const DEFAULT_BOARD_SIZE: usize = 7;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Board<const SIZE: usize = DEFAULT_BOARD_SIZE> {
    board: [[Color; SIZE]; SIZE],
    next_color: Color,
    winner: Color,
}

impl<const SIZE: usize> Board<SIZE> {
    pub fn new() -> Board<SIZE> {
        Board {
            board: [[Color::None; SIZE]; SIZE],
            next_color: Color::White,
            winner: Color::None,
        }
    }

    pub fn size(&self) -> usize {
        SIZE
    }

    pub fn play(&mut self, x: usize, y: usize) {
        self.board[x][y] = self.next_color;
        self.next_color = self.next_color.opponent();
    }

    pub fn get_board(&self) -> &[[Color; SIZE]; SIZE] {
        &self.board
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        if x >= SIZE || y >= SIZE {
            return false;
        }

        self.board[x][y] == Color::None
    }

    pub fn next_color(&self) -> Color {
        self.next_color
    }

    fn reach(
        &self,
        player: Color,
        distance: usize,
        distances: Vec<Vec<Distance>>,
    ) -> (Vec<Vec<Distance>>, bool) {
        //println!("reach called with dist of {}", dist);
        let mut new_distances = distances.clone();
        let mut changed = false;

        for i in 0..SIZE {
            for j in 0..SIZE {
                if let Distance::Reachable(d) = distances[i][j] {
                    if d == distance {
                        for neighbor in Cell::new(i as i32, j as i32).neighbors(SIZE) {
                            if let Distance::Unexplored =
                                distances[neighbor.x as usize][neighbor.y as usize]
                            {
                                changed = true;
                                match self.board[neighbor.x as usize][neighbor.y as usize] {
                                    Color::None => {
                                        new_distances[neighbor.x as usize][neighbor.y as usize] =
                                            Distance::Reachable(distance + 1)
                                    }
                                    p => {
                                        if p == player {
                                            new_distances[neighbor.x as usize]
                                                [neighbor.y as usize] =
                                                Distance::Reachable(distance);
                                            (new_distances, _) =
                                                self.reach(player, distance, new_distances);
                                        } else {
                                            new_distances[neighbor.x as usize]
                                                [neighbor.y as usize] = Distance::Unreachable;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        (new_distances, changed)
    }

    pub fn missing_move_to_win(&self, color: Color) -> Option<usize> {
        let is_finished = |distances: &Vec<Vec<Distance>>| {
            for i in 0..SIZE {
                for row in distances.iter() {
                    if let Distance::Unexplored = row[i] {
                        return false;
                    }
                }
            }
            true
        };

        match color {
            Color::Black => {
                let mut distances: Vec<Vec<Distance>> =
                    vec![vec![Distance::Unexplored; SIZE]; SIZE];
                distances[0] = self.board[0]
                    .iter()
                    .map(|x| match x {
                        Color::None => Distance::Reachable(1),
                        _ => {
                            if *x == color {
                                Distance::Reachable(0)
                            } else {
                                Distance::Unreachable
                            }
                        }
                    })
                    .collect();

                // reach all cells
                let mut dist = 0;
                while !is_finished(&distances) {
                    //println!("{} {:?}", dist, distances);
                    let changed;
                    (distances, changed) = self.reach(color, dist, distances);
                    dist += 1;
                    if !changed && dist != 1 {
                        break;
                    }
                }

                //println!("{:?}", distances);

                // get the minimum distance to the last row
                distances[SIZE - 1]
                    .iter()
                    .filter_map(|x| match x {
                        Distance::Reachable(d) => Some(d),
                        _ => None,
                    })
                    .min()
                    .cloned()
            }
            Color::White => {
                let mut distances: Vec<Vec<Distance>> =
                    vec![vec![Distance::Unexplored; SIZE]; SIZE];

                //change first columns of distances to 1
                for (y, row) in distances.iter_mut().enumerate() {
                    row[0] = match self.board[y][0] {
                        Color::None => Distance::Reachable(1),
                        _ => {
                            if self.board[y][0] == color {
                                Distance::Reachable(0)
                            } else {
                                Distance::Unreachable
                            }
                        }
                    };
                }

                // reach all cells
                let mut dist = 0;
                while !is_finished(&distances) {
                    //println!("{} {:?}", dist, distances);
                    let changed;
                    (distances, changed) = self.reach(color, dist, distances);
                    dist += 1;
                    if !changed && dist != 1 {
                        break;
                    }
                }

                //println!("{:?}", distances);

                // get the minimum distance to the last column
                distances
                    .iter()
                    .map(|row| row[SIZE - 1])
                    .filter_map(|x| match x {
                        Distance::Reachable(d) => Some(d),
                        _ => None,
                    })
                    .min()
            }
            _ => panic!("Player::None has no missing move to win"),
        }
    }

    pub fn get_dist_matrix(&self, color: Color) -> Vec<Vec<Distance>> {
        let mut zero_queue = VecDeque::with_capacity(SIZE * SIZE);

        let mut distances: Vec<Vec<Distance>> = match color {
            Color::Black => self
                .board
                .iter()
                .enumerate()
                .map(|(i, rows)| {
                    if i == 0 {
                        rows.iter()
                            .enumerate()
                            .map(|(j, value)| match value {
                                Color::Black => {
                                    zero_queue.push_back(Cell::new(i as i32, j as i32));
                                    Distance::Reachable(0)
                                }
                                Color::White => Distance::Unreachable,
                                Color::None => {
                                    zero_queue.push_back(Cell::new(i as i32, j as i32));
                                    Distance::Reachable(1)
                                }
                            })
                            .collect()
                    } else {
                        rows.iter()
                            .map(|value| match value {
                                Color::White => Distance::Unreachable,
                                _ => Distance::Unexplored,
                            })
                            .collect()
                    }
                })
                .collect(),
            Color::White => self
                .board
                .iter()
                .enumerate()
                .map(|(i, rows)| {
                    rows.iter()
                        .enumerate()
                        .map(|(j, value)| {
                            if j == 0 {
                                match value {
                                    Color::White => {
                                        zero_queue.push_back(Cell::new(i as i32, j as i32));
                                        Distance::Reachable(0)
                                    }
                                    Color::Black => Distance::Unreachable,
                                    Color::None => {
                                        zero_queue.push_back(Cell::new(i as i32, j as i32));
                                        Distance::Reachable(1)
                                    }
                                }
                            } else {
                                match value {
                                    Color::Black => Distance::Unreachable,
                                    _ => Distance::Unexplored,
                                }
                            }
                        })
                        .collect()
                })
                .collect(),
            _ => unreachable!(),
        };

        if zero_queue.is_empty() {
            return distances;
        }

        // Find all cell where Reachable without moving (Distance::Reachable(0))
        let mut changed = true;
        while changed {
            changed = false;
            let cell = zero_queue.front().unwrap();
            for neighbor in cell.neighbors(SIZE) {
                if self.board[neighbor.x as usize][neighbor.y as usize] == color
                    && distances[neighbor.x as usize][neighbor.y as usize] == Distance::Unexplored
                {
                    zero_queue.push_back(neighbor);
                    distances[neighbor.x as usize][neighbor.y as usize] = Distance::Reachable(0);
                    changed = true;
                }
            }

            if zero_queue.is_empty() {
                break;
            }
        }

        // Iterate through depths
        let mut queue = zero_queue;
        while !queue.is_empty() {
            let cell = queue.pop_front().unwrap();
            if let Some(depth) = distances[cell.x as usize][cell.y as usize].into() {
                for neighbor in cell.neighbors(SIZE) {
                    if distances[neighbor.x as usize][neighbor.y as usize] == Distance::Unexplored {
                        if self.board[neighbor.x as usize][neighbor.y as usize] == color {
                            queue.push_front(neighbor);
                            distances[neighbor.x as usize][neighbor.y as usize] =
                                Distance::Reachable(depth);
                        } else {
                            queue.push_back(neighbor);
                            distances[neighbor.x as usize][neighbor.y as usize] =
                                Distance::Reachable(depth + 1);
                        }
                    }
                }
            }
        }

        distances
    }

    pub fn missing_move_to_win2(&self, color: Color) -> Option<usize> {
        let distances = self.get_dist_matrix(color);

        match color {
            Color::Black => distances
                .iter()
                .last()
                .unwrap()
                .iter()
                .min()
                .unwrap()
                .to_owned()
                .into(),
            Color::White => distances
                .iter()
                .map(|row| row.iter().last().unwrap())
                .min()
                .unwrap()
                .to_owned()
                .into(),
            _ => unreachable!(),
        }
    }

    pub fn play_random_move(&mut self) {
        let possible_moves = self.possible_moves();

        if possible_moves.is_empty() {
            return;
        }

        let mut rng = rand::rng();
        let index = rng.random_range(0..possible_moves.len());
        let (x, y) = possible_moves[index];
        self.play(x, y);
    }

    #[allow(unused)]
    pub fn random_board(nb_moves: usize) -> Board<SIZE> {
        let mut board = Board::new();
        let mut color = Color::Black;

        for _ in 0..nb_moves {
            board.play_random_move();
            color = color.opponent();
        }

        board
    }

    #[allow(unused)]
    pub fn is_win(&self) -> bool {
        self.missing_move_to_win2(self.next_color.opponent()) == Some(0)
    }

    #[allow(unused)]
    pub fn winner(&self) -> Option<Color> {
        if self.winner != Color::None {
            None
        } else if self.missing_move_to_win2(Color::Black) == Some(0) {
            Some(Color::Black)
        } else if self.missing_move_to_win2(Color::White) == Some(0) {
            Some(Color::White)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn is_full(&self) -> bool {
        self.first_possible_move().is_none()
    }

    pub fn possible_moves(&self) -> Vec<(usize, usize)> {
        self.board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| **cell == Color::None)
                    .map(move |(j, _)| (i, j))
            })
            .collect()
    }

    pub fn first_possible_move(&self) -> Option<(usize, usize)> {
        self.possible_moves().first().copied()
    }

    #[allow(unused)]
    pub fn is_finished(&self) -> bool {
        !self.winner().is_none()
    }
}

impl<const SIZE: usize> Default for Board<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> fmt::Display for Board<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_column_labels(f, SIZE, 0)?;

        for row in 0..SIZE {
            write_row(f, self, row)?;
        }

        write_column_labels(f, SIZE, SIZE + 1)
    }
}

pub fn write_column_labels(
    f: &mut fmt::Formatter,
    board_size: usize,
    indent: usize,
) -> fmt::Result {
    write_indent(f, indent)?;
    write!(f, " ")?;

    for column in 0..board_size {
        write!(f, " {} ", column + 1)?;
    }

    writeln!(f)
}

pub fn write_row<const SIZE: usize>(
    f: &mut fmt::Formatter,
    board: &Board<SIZE>,
    row: usize,
) -> fmt::Result {
    write_indent(f, row)?;
    write!(f, "{:2}\\", row + 1)?;

    for column in 0..board.size() {
        if column > 0 {
            write!(f, "  ")?;
        }
        write!(f, "{}", board.get_board()[row][column].to_char())?;
    }

    writeln!(f, "\\{:2}", row + 1)
}

pub fn write_indent(f: &mut fmt::Formatter, length: usize) -> fmt::Result {
    write!(f, "{}", " ".repeat(length))
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let mut board = Board::<2>::new();
        board.play(0, 0);
        board.play(0, 1);
        board.play(1, 1);
        board.play(1, 0);

        assert!(board.is_win());
        assert!(board.is_full());
    }

    #[test]
    fn is_game_over() {
        let board = Board::<4>::new();
        assert_eq!(board.missing_move_to_win(Color::White), Some(4));

        assert!(!board.is_win());
    }

    #[test]
    fn first_possible_move() {
        let mut board = Board::<2>::new();
        board.play(0, 0);

        assert_eq!(board.first_possible_move().unwrap(), (0, 1));
    }

    #[test]
    fn is_game_over1() {
        let mut board = Board::<4>::new();
        board.play(3, 0);
        board.play(0, 0);
        board.play(3, 1);
        board.play(0, 1);
        board.play(2, 2);
        board.play(1, 0);
        board.play(1, 2);
        board.play(1, 1);
        board.play(0, 3);

        assert_eq!(board.missing_move_to_win(Color::White), Some(0));

        assert!(board.is_win());
    }

    #[test]
    fn is_game_over2() {
        let mut board = Board::<4>::new();
        board.play(3, 0);
        board.play(0, 0);
        board.play(3, 1);
        board.play(0, 1);
        board.play(2, 2);
        board.play(1, 0);
        board.play(1, 3);

        assert_eq!(board.missing_move_to_win(Color::White), Some(0));

        assert!(board.is_win());
    }

    #[test]
    fn missing_moves() {
        let board = Board::<4>::new();

        assert_eq!(board.missing_move_to_win(Color::White), Some(4));
        assert_eq!(board.missing_move_to_win(Color::Black), Some(4));

        let mut board = Board::<4>::new();
        board.play(3, 0);
        board.play(0, 0);
        board.play(3, 1);

        assert_eq!(board.missing_move_to_win(Color::White), Some(2));
        assert_eq!(board.missing_move_to_win(Color::Black), Some(4));
    }

    #[test]
    fn dist_matrix() {
        let mut board = Board::<2>::new();
        board.play(0, 0);

        let dist_matrix_white = board.get_dist_matrix(Color::White);
        assert_eq!(
            dist_matrix_white,
            vec![
                vec![Distance::Reachable(0), Distance::Reachable(1)],
                vec![Distance::Reachable(1), Distance::Reachable(2)]
            ]
        );

        let dist_matrix_black = board.get_dist_matrix(Color::Black);
        assert_eq!(
            dist_matrix_black,
            vec![
                vec![Distance::Unreachable, Distance::Reachable(1)],
                vec![Distance::Reachable(2), Distance::Reachable(2)]
            ]
        );
    }

    #[test]
    fn missing_moves_to_win2() {
        let mut board = Board::<4>::new();

        assert_eq!(board.missing_move_to_win2(Color::White), Some(4));
        assert_eq!(board.missing_move_to_win2(Color::Black), Some(4));

        board.play(3, 0);
        board.play(0, 0);
        board.play(3, 1);

        assert_eq!(board.missing_move_to_win2(Color::White), Some(2));
        assert_eq!(board.missing_move_to_win2(Color::Black), Some(4));
    }

    #[test]
    fn possible_moves() {
        let mut board = Board::<2>::new();
        board.play(0, 0);

        assert_eq!(board.possible_moves(), vec![(0, 1), (1, 0), (1, 1)]);
    }
}
