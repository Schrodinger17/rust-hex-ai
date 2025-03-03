use std::{collections::HashMap, rc::Rc};

use crate::{
    color::Color,
    game::Game,
    log::{LogFlag, LogLevel},
    player::Player,
};

#[warn(dead_code)]
#[derive(Default)]
pub struct Tournament {
    players: Vec<Rc<Player>>,
    games: Vec<Game>,
    results: Vec<Vec<usize>>,
    nb_games: usize,
    log_level: Rc<LogLevel>,
}

#[allow(unused)]
impl Tournament {
    pub fn new() -> Tournament {
        Tournament::default()
    }

    pub fn set_players(&mut self, players: Vec<Rc<Player>>) {
        self.players = players;
    }

    pub fn create_games(&mut self, nb_games: usize) {
        self.nb_games = nb_games;
        for player1 in self.players.iter() {
            for player2 in self.players.iter() {
                for _ in 0..nb_games {
                    let mut players: HashMap<Color, Rc<Player>> = HashMap::new();
                    players.insert(Color::White, player1.clone());
                    players.insert(Color::Black, player2.clone());

                    let mut game = Game::new(players);
                    game.set_log_level(self.log_level.clone());
                    self.games.push(game);
                }
            }
        }

        let n = self.players.len();
        self.results = vec![vec![0; n]; n];
    }

    pub fn play(&mut self) {
        let n = self.players.len();
        for id in 0..self.games.len() {
            let cell_id = id / self.nb_games;

            self.games[id].play();
            if self.games[id].winner() == Color::White {
                self.results[cell_id / n][cell_id % n] += 1
            }

            if self.log_level.is(LogFlag::MatchResult) {
                println!("{}", self.games[id].board());
                println!("Game {}/{} finished.", id + 1, self.games.len());
            }
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
