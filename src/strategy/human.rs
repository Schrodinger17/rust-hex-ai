use std::time::Duration;

use crate::{board::Board, color::Color};

use super::StrategyTrait;


#[derive(Clone)]
pub struct Human {
    #[allow(dead_code)]
    name: String,
}


impl StrategyTrait for Human {
    #[allow(unused_variables)]
    fn next_move(&self, board: &Board, color: Color, duration: Option<Duration>) -> (usize, usize) {
        Human::ask_coord(color, &board)
    }
}

impl Human {
    #[allow(dead_code)]
    fn new(name: String) -> Human {
        Human {
            name,
        }
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
        println!("{}'s turn", color.to_string());
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match Human::read_coord(&input) {
            Some((x, y)) => {
                if board.is_valid(x-1, y-1) {
                    (x, y)
                } else {
                    println!("Invalid move, try again ('x y')");
                    Human::ask_coord(color, board)
                }
            },
            None => Human::ask_coord(color, board)
    }
}
}
