#![allow(unused_imports)]
mod player;

use std::collections::HashMap;

use player::Player;
use std::rc::Rc;
use std::time::Duration;

mod board;
mod game;
use game::Game;
mod cell;
mod display;
mod distance;
mod gui;
mod tournament;
use tournament::Tournament;

mod color;
mod score;

use color::Color;
mod strategy;

mod evaluation;
use evaluation::evaluation1::Evaluation1;
use strategy::alpha_beta::AlphaBeta;
use strategy::alpha_beta_2::AlphaBeta2;
use strategy::alpha_beta_4::AlphaBeta4;
use strategy::human::Human;
use strategy::mini_max::MiniMax;
use strategy::random::Random;
//use strategy::mcts::MCTS;
mod best_list;
mod test;

#[allow(clippy::vec_init_then_push)]
fn main() {
    let duration = Duration::from_millis(100);

    let mut players: HashMap<Color, Rc<Player>> = HashMap::new();
    players.insert(
        Color::Black,
        Rc::new(Player::new(
            "AlphaBeta_2".to_string(),
            Rc::new(AlphaBeta2::new(Rc::new(Evaluation1::new()), 20, None)),
            Some(duration),
        )),
    );

    players.insert(
        Color::White,
        Rc::new(Player::new(
            "AlphaBeta_4".to_string(),
            Rc::new(AlphaBeta4::new(Rc::new(Evaluation1::new()), 20, None)),
            Some(duration),
        )),
    );

    let mut hex = Game::new(5, players);
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
