mod player;
use std::collections::HashMap;

use std::sync::Arc;
use std::time::Duration;
use player::Player;

mod board;
mod game;
use game::Game;
mod cell;
mod distance;
mod gui;
mod display;
//mod tournament;

mod color;
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
use strategy::mini_max::MiniMax;
#[allow(unused_imports)]
use strategy::human::Human;
#[allow(unused_imports)]
use strategy::random::Random;

fn main() {
    let mut players: HashMap<Color, Player> = HashMap::new();
    
    let duration = Duration::from_millis(1000);
    
    players.insert(
        Color::White,
        Player::new(
            "AlphaBeta".to_string(),
            Arc::new(AlphaBeta::new(
                Arc::new(Evaluation1::new()),
                10,
                None,
            )),
            Some(duration),
        ),
    );
    
    players.insert(
        Color::Black,
        Player::new(
            "AlphaBeta2".to_string(),
            Arc::new(AlphaBeta2::new(
                Arc::new(Evaluation1::new()),
                10,
                None,
            )),
            Some(duration),
        ),
    );    
    let mut hex = Game::new(9, players);
    hex.set_duration(duration);
    hex.play();
    

    /*
    let mut strategies = Vec::new();
    //strategies.push(Strategy::Random(strategy::random::Random::new()));
    //strategies.push(Strategy::Random(strategy::random::Random::new()));
    
    strategies.push(Strategy::AlphaBeta(strategy::alpha_beta::AlphaBeta::new(
        Evaluation::Evaluation1(evaluation::evaluation1::Evaluation1::new()),
        3,
        None,
    )));

    strategies.push(Strategy::AlphaBetaV2(strategy::alpha_beta_2::AlphaBeta2::new(
        Evaluation::Evaluation1(evaluation::evaluation1::Evaluation1::new()),
        4,
        None,
    )));

    let mut tournament = tournament::Tournament::new(strategies, 5, 10);
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