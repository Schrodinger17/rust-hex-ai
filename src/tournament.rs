use std::{collections::HashMap, rc::Rc};

use crate::{color::Color, game::Game, player::Player};

#[warn(dead_code)]
pub struct Tournament {
    players: Vec<Rc<Player>>,
    games: Vec<Game>,
    results: Vec<Vec<usize>>,
    nb_games: usize,
}

#[allow(dead_code)]
impl Tournament {
    pub fn new(players: Vec<Rc<Player>>, board_size: usize, nb_games: usize) -> Tournament {
        let mut games = Vec::new();
        for player1 in players.iter() {
            for player2 in players.iter() {
                for _ in 0..nb_games {
                    let mut players: HashMap<Color, Rc<Player>> = HashMap::new();
                    players.insert(Color::White, player1.clone());
                    players.insert(Color::Black, player2.clone());

                    games.push(Game::new(board_size, players));
                }
            }
        }

        let n = players.len();

        Tournament {
            players,
            games,
            results: vec![vec![0; n]; n],
            nb_games,
        }
    }

    pub fn play(&mut self) {
        let n = self.players.len();
        for id in 0..self.games.len() {
            let cell_id = id / self.nb_games;
            self.games[id].set_display(false);

            self.games[id].play();
            if self.games[id].winner() == Color::White {
                self.results[cell_id / n][cell_id % n] += 1
            }
            println!("{}", self.games[id].board());
            println!("Game {}/{} finished.", id + 1, self.games.len());
        }
    }

    pub fn print_results(&self) {
        let n = self.players.len();
        println!("Tournament results:");
        println!("{} players, {} games.", self.players.len(), self.nb_games);
        print!("W\\B ");
        for id in 0..n {
            print!("{:2} ", id);
        }
        println!();
        for id1 in 0..n {
            print!("{:2} :", id1);
            for id2 in 0..n {
                print!("{:2} ", self.results[id1][id2]);
            }
            println!();
        }
        println!();
    }
}

//test
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        for id in 0..4 {
            println!("{} {} {}", id, id / 2, id % 2);
        }
    }
}
