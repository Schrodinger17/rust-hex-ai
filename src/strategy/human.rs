use std::time::Duration;

use crate::{board::Board, color::Color};

use super::Strategy;

#[derive(Clone)]
pub struct Human {
    #[allow(unused)]
    name: String,
}

impl Strategy for Human {
    #[allow(unused_variables)]
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        Human::ask_coord(color, board)
    }
}

impl Human {
    #[allow(unused)]
    pub fn new(name: String) -> Human {
        Human { name }
    }

    fn read_coord(input: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                return Some((num1, num2));
            }
        }

        None
    }

    fn ask_coord(color: Color, board: &Board) -> (usize, usize) {
        let mut input = String::new();
        println!("{}'s turn", color);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match Human::read_coord(&input) {
            Some((x, y)) => {
                if x > 0 && y > 0 && board.is_valid(x - 1, y - 1) {
                    (x - 1, y - 1)
                } else {
                    println!("Invalid move, try again ('x y')");
                    Human::ask_coord(color, board)
                }
            }
            None => Human::ask_coord(color, board),
        }
    }
}
