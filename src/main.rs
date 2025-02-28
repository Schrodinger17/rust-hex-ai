#![allow(unused_imports)]

mod best_list;
mod board;
mod cell;
mod color;
mod display;
mod distance;
mod evaluation;
mod game;
mod gui;
mod log;
mod player;
mod score;
mod strategy;
mod tournament;

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use color::Color;
use evaluation::*;
use game::Game;
use player::Player;
use strategy::*;
use tournament::Tournament;

#[allow(clippy::vec_init_then_push)]
fn main() {
    let duration = Duration::from_millis(100);

    let mut players: HashMap<Color, Rc<Player>> = HashMap::new();
    players.insert(
        Color::Black,
        Rc::new(Player::new(
            "AlphaBeta_4".to_string(),
            Rc::new(AlphaBeta4::new(Rc::new(Evaluation1::new()), 20, None)),
            Some(duration),
        )),
    );

    players.insert(
        Color::White,
        Rc::new(Player::new(
            "AlphaBeta_4".to_string(),
            Rc::new(AlphaBeta4::new(Rc::new(Evaluation2::new()), 20, None)),
            Some(duration),
        )),
    );

    let mut hex = Game::new(6, players);
    hex.set_duration(duration);
    hex.play();

    /*
    
    let mut strategies = Vec::new();

    strategies.push(Rc::new(Player::new(
        "Random".to_string(),
        Rc::new(Random::new()),
        None,
    )));

    strategies.push(Rc::new(Player::new(
        "MiniMax".to_string(),
        Rc::new(MiniMax::new(Rc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    strategies.push(Rc::new(Player::new(
        "AlphaBeta".to_string(),
        Rc::new(AlphaBeta::new(Rc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    strategies.push(Rc::new(Player::new(
        "AlphaBeta2".to_string(),
        Rc::new(AlphaBeta2::new(Rc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    let mut tournament = Tournament::new(strategies, 7, 10);
    tournament.play();
    tournament.print_results();
    */

    /*
    let mut strategies = Vec::new();

    strategies.push(Rc::new(Player::new(
        "AlphaBeta2".to_string(),
        Rc::new(AlphaBeta2::new(Rc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    strategies.push(Rc::new(Player::new(
        "AlphaBeta4".to_string(),
        Rc::new(AlphaBeta4::new(Rc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));
    
    let mut tournament = Tournament::new(strategies, 5, 4);
    tournament.play();
    tournament.print_results();
    */
}

/*
Tournament results:
5 players, 10 games.
W\B  0  1  2  3  4
 0 : 7  0  0  0  1
 1 :10  0  0  0  0
 2 :10 10 10 10 10
 3 :10 10 10 10 10
 4 :10 10 10 10 10
 */

/*
 Game 90/90 finished.
Tournament results:
3 players, 10 games.
W\B  0  1  2
 0 : 5  0  0
 1 :10  6  5
 2 :10  9  9
  */
