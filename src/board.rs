use core::fmt;
use std::hash::Hash;

use rand::Rng;

use crate::cell::Cell;
use crate::distance::Distance;
use crate::color::Color;
use crate::display::{write_column_labels, write_row};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub board: Vec<Vec<Color>>,
    pub size: usize,
    pub score: f64,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            board: vec![vec![Color::None; size]; size],
            size: size,
            score: 0.0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.board[x][y] = color;
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        if x >= self.size || y >= self.size {
            return false;
        }

        match self.board[x][y] {
            Color::None => true,
            _ => false,
        }
    }

    fn reach(
        &self,
        player: Color,
        dist: usize,
        dists: Vec<Vec<Distance>>,
    ) -> (Vec<Vec<Distance>>, bool) {
        //println!("reach called with dist of {}", dist);
        let mut new_dists = dists.clone();
        let mut changed = false;

        for i in 0..self.size {
            for j in 0..self.size {
                if let Distance::Reachable(d) = dists[i][j] {
                    if d == dist {
                        for neighbor in Cell::new(i as i32, j as i32).neighbors(self.size) {
                            if let Distance::Unexplored =
                                dists[neighbor.x as usize][neighbor.y as usize]
                            {
                                changed = true;
                                match self.board[neighbor.x as usize][neighbor.y as usize] {
                                    Color::None => {
                                        new_dists[neighbor.x as usize][neighbor.y as usize] =
                                            Distance::Reachable(dist + 1)
                                    }
                                    p => {
                                        if p == player {
                                            new_dists[neighbor.x as usize][neighbor.y as usize] =
                                                Distance::Reachable(dist);
                                            (new_dists, _) = self.reach(player, dist, new_dists);
                                        } else {
                                            new_dists[neighbor.x as usize][neighbor.y as usize] =
                                                Distance::Unreachable;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        (new_dists, changed)
    }

    pub fn missing_move_to_win(&self, color: Color) -> usize {
        let is_finished = |dists: &Vec<Vec<Distance>>| {
            for i in 0..self.size {
                for j in 0..self.size {
                    match dists[i][j] {
                        Distance::Unexplored => return false,
                        _ => (),
                    }
                }
            }
            true
        };

        match color {
            Color::Black => {
                let mut dists: Vec<Vec<Distance>> =
                    vec![vec![Distance::Unexplored; self.size]; self.size];
                dists[0] = self.board[0]
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
                while !is_finished(&dists) {
                    //println!("{} {:?}", dist, dists);
                    let changed;
                    (dists, changed) = self.reach(color, dist, dists);
                    dist += 1;
                    if !changed && dist != 1 {
                        break;
                    }
                }

                //println!("{:?}", dists);

                // get the minimum distance to the last row
                match dists[self.size - 1]
                    .iter()
                    .filter_map(|x| match x {
                        Distance::Reachable(d) => Some(d),
                        _ => None,
                    })
                    .min()
                {
                    Some(d) => *d,
                    None => usize::MAX,
                }
            }
            Color::White => {
                let mut dists: Vec<Vec<Distance>> =
                    vec![vec![Distance::Unexplored; self.size]; self.size];

                //change first columns of dists to 1
                for y in 0..self.size {
                    dists[y][0] = match self.board[y][0] {
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
                while !is_finished(&dists) {
                    //println!("{} {:?}", dist, dists);
                    let changed;
                    (dists, changed) = self.reach(color, dist, dists);
                    dist += 1;
                    if !changed && dist != 1 {
                        break;
                    }
                }

                //println!("{:?}", dists);

                // get the minimum distance to the last column
                match dists
                    .iter()
                    .map(|row| row[self.size - 1])
                    .filter_map(|x| match x {
                        Distance::Reachable(d) => Some(d),
                        _ => None,
                    })
                    .min()
                {
                    Some(d) => d,
                    None => usize::MAX,
                }
            }
            _ => panic!("Player::None has no missing move to win"),
        }
    }
    
    pub fn play_random_move(&mut self, color: Color) {
        let possible_moves = self.possible_moves();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..possible_moves.len());
        let (x, y) = possible_moves[index];
        self.set(x, y, color);
    }

    pub fn is_win(&self, color: Color) -> bool {
        self.missing_move_to_win(color) == 0
    }

    pub fn is_full(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.board[i][j] == Color::None {
                    return false;
                }
            }
        }
        true
    }

    pub fn possible_moves(&self) -> Vec<(usize, usize)> {
        self.board
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| **cell == Color::None)
                    .map(move |(j, _)| (i, j))
            })
            .flatten()
            .collect()
    }

    pub fn a_possible_move(&self) -> (usize, usize) {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.board[i][j] == Color::None {
                    return (i, j);
                }
            }
        }
        panic!("No possible move")
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_column_labels(f, self.size(), 0)?;

        for row in 0..self.size() {
            write_row(f, self, row)?;
        }

        write_column_labels(f, self.size(), (self.size() + 1) as usize)
    }
}

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.size.hash(state);
    }
}

impl Eq for Board {
    
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let mut board = Board::new(2);
        board.set(0, 0, Color::White);
        board.set(0, 1, Color::Black);
        board.set(1, 0, Color::Black);
        board.set(1, 1, Color::White);
        //println!("{}", board.is_win(Color::White));
        //println!("{}", board.is_win(Color::Black));
        assert_eq!(board.is_win(Color::White), false);
        assert_eq!(board.is_win(Color::Black), true);
        assert_eq!(board.is_full(), true);
    }

    #[test]
    fn is_game_over() {
        let board = Board::new(4);
        println!("{}", board);
        println!("{}", board.missing_move_to_win(Color::White));

        assert_eq!(board.is_win(Color::White), false);
    }

    #[test]
    fn is_game_over1() {
        let mut board = Board::new(4);
        board.set(3, 0, Color::White);
        board.set(3, 1, Color::White);
        board.set(2, 2, Color::White);
        board.set(1, 2, Color::White);
        board.set(0, 3, Color::White);
        println!("{}", board);
        println!("{}", board.missing_move_to_win(Color::White));

        assert_eq!(board.is_win(Color::White), true);
    }

    #[test]
    fn is_game_over2() {
        let mut board = Board::new(4);
        board.set(3, 0, Color::White);
        board.set(3, 1, Color::White);
        board.set(2, 2, Color::White);
        board.set(1, 3, Color::White);
        println!("{}", board);
        println!("{}", board.missing_move_to_win(Color::White));

        assert_eq!(board.is_win(Color::White), true);
    }

    #[test]
    fn missing_moves() {
        let board = Board::new(4);
        println!("{}", board);

        assert_eq!(board.missing_move_to_win(Color::White), 4);
        assert_eq!(board.missing_move_to_win(Color::Black), 4);
    }

    #[test]
    fn missing_moves2() {
        let mut board = Board::new(4);
        board.set(3, 0, Color::White);
        board.set(3, 1, Color::White);
        println!("{}", board);

        assert_eq!(board.missing_move_to_win(Color::White), 2);
        assert_eq!(board.missing_move_to_win(Color::Black), 4);
    }

    #[test]
    fn reach_test() {
        let board = Board::new(11);
        //board.set(3, 0, Color::White);
        //board.set(3, 1, Color::White);
        println!("{}", board);

        //calculate call time
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            board.missing_move_to_win(Color::White);
        }

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }

    #[test]
    fn reach_test2() {
        let mut board = Board::new(11);
        board.set(3, 0, Color::White);
        board.set(3, 1, Color::White);
        board.set(2, 2, Color::White);
        board.set(1, 2, Color::White);
        board.set(0, 3, Color::White);
        board.set(0, 4, Color::White);
        board.set(0, 5, Color::White);
        board.set(0, 6, Color::White);
        board.set(1, 6, Color::White);
        board.set(2, 6, Color::White);
        board.set(3, 6, Color::White);
        board.set(4, 6, Color::White);
        board.set(5, 5, Color::White);
        board.set(6, 5, Color::White);
        board.set(6, 6, Color::White);
        board.set(6, 7, Color::White);
        board.set(6, 8, Color::White);
        board.set(6, 9, Color::White);
        board.set(6, 10, Color::White);
        println!("{}", board);

        //calculate call time
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            board.missing_move_to_win(Color::White);
        }

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }


    #[test]
    fn possible_moves() {
        let mut board = Board::new(2);
        board.set(0, 0, Color::White);
        println!("{}", board);

        assert_eq!(board.possible_moves(), vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn spped_test() {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Student {
            name: String,
            gpa: f32,
        }
        {
            let students = vec![
                "Bogdan 3.1",
                "Wallace 2.3",
                "Lidiya 3.5",
                "Kyle 3.9",
                "Anatoliy 4.0",
            ];

            let start = std::time::Instant::now();

            for _ in 0..1000 {
                #[allow(unused_variables)]
                let good_students: Vec<Student> = students
                    .iter()
                    .map(|s| {
                        let mut s = s.split(' ');
                        let name = s.next()?.to_owned();
                        let gpa = s.next()?.parse::<f32>().ok()?;

                        Some(Student { name, gpa })
                    })
                    .flatten()
                    .filter(|s| s.gpa >= 3.5)
                    .collect();
            }

            let duration = start.elapsed();
            println!("Time elapsed in expensive_function() is: {:?}", duration);
        }
        // ----------------------------------
        {
            let students = vec![
                "Bogdan 3.1",
                "Wallace 2.3",
                "Lidiya 3.5",
                "Kyle 3.9",
                "Anatoliy 4.0",
            ];

            let start = std::time::Instant::now();

            for _ in 0..1000 {
                let mut good_students = vec![];

                for s in &students {
                    let mut s = s.split(' ');
                    let name = s.next();
                    let gpa = s.next();

                    if name.is_some() && gpa.is_some() {
                        let name = name.unwrap().to_owned();
                        let gpa = gpa.unwrap().to_owned();

                        let gpa = gpa.parse::<f32>();

                        if gpa.is_ok() {
                            let gpa = gpa.unwrap();
                            if gpa >= 3.5 {
                                good_students.push(Student { name, gpa });
                            }
                        }
                    }
                }
            }

            let duration = start.elapsed();
            println!("Time elapsed in expensive_function() is: {:?}", duration);
        }
    }
}
