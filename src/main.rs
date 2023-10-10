mod player;

#[allow(unused_imports)]
use std::collections::HashMap;

use player::Player;
use std::sync::Arc;
use std::time::Duration;

mod board;
mod game;
#[allow(unused_imports)]
use game::Game;
mod cell;
mod display;
mod distance;
mod gui;
mod tournament;
#[allow(unused_imports)]
use tournament::Tournament;

mod color;
mod score;
#[allow(unused_imports)]
use color::Color;

mod strategy;

mod evaluation;
#[allow(unused_imports)]
use evaluation::evaluation1::Evaluation1;
#[allow(unused_imports)]
use strategy::alpha_beta::AlphaBeta;
#[allow(unused_imports)]
use strategy::alpha_beta_2::AlphaBeta2;
#[allow(unused_imports)]
use strategy::human::Human;
#[allow(unused_imports)]
use strategy::mini_max::MiniMax;
#[allow(unused_imports)]
use strategy::random::Random;

mod best_list;
mod test;

fn main() {
    let duration = Duration::from_millis(1000);

    let mut players: HashMap<Color, Arc<Player>> = HashMap::new();
    players.insert(
        Color::White,
        Arc::new(Player::new(
            "MiniMax".to_string(),
            Arc::new(MiniMax::new(Arc::new(Evaluation1::new()), 20, None)),
            Some(duration),
        )),
    );

    players.insert(
        Color::Black,
        Arc::new(Player::new(
            "AlphaBeta_2".to_string(),
            Arc::new(AlphaBeta2::new(Arc::new(Evaluation1::new()), 20, None)),
            Some(duration),
        )),
    );

    let mut hex = Game::new(3, players);
    hex.set_duration(duration);
    hex.play();

    /*
    let mut strategies = Vec::new();

    strategies.push(Arc::new(Player::new(
        "Random".to_string(),
        Arc::new(Random::new()),
        None,
    )));

    strategies.push(Arc::new(Player::new(
        "MiniMax".to_string(),
        Arc::new(MiniMax::new(Arc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    strategies.push(Arc::new(Player::new(
        "AlphaBeta".to_string(),
        Arc::new(AlphaBeta::new(Arc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    strategies.push(Arc::new(Player::new(
        "AlphaBeta2".to_string(),
        Arc::new(AlphaBeta2::new(Arc::new(Evaluation1::new()), 10, None)),
        Some(duration),
    )));

    let mut tournament = Tournament::new(strategies, 7, 10);
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
