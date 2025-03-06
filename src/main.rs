#![allow(unused_imports)]

mod best_list;
mod board;
mod cell;
mod color;
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
use log::{LogFlag, LogLevel};
use player::Player;
use strategy::*;
use tournament::Tournament;

#[allow(clippy::vec_init_then_push)]
fn main() {
    let log_level = Rc::new(
        LogLevel::new()
            .add(LogFlag::GameResult)
            .add(LogFlag::Position)
            .add(LogFlag::SearchDepth)
            .add(LogFlag::Moves)
            .add(LogFlag::Score)
            .to_owned(),
    );

    let duration = Duration::from_millis(1000);

    let mut players: HashMap<Color, Rc<Player>> = HashMap::new();
    players.insert(
        Color::Black,
        Rc::new(Player::new(
            "AlphaBeta_4".to_string(),
            Rc::new(AlphaBeta4::new(
                Rc::new(Evaluation3::new()),
                20,
                log_level.clone(),
            )),
            Some(duration),
        )),
    );

    /*
    players.insert(
        Color::Black,
        Rc::new(Player::new(
            "AlphaBeta_4".to_string(),
            Rc::new(Human::new("Me".to_string())),
            Some(duration),
        )),
    );
    */

    players.insert(
        Color::White,
        Rc::new(Player::new(
            "AlphaBeta_4".to_string(),
            Rc::new(AlphaBeta4::new(
                Rc::new(Evaluation3::new()),
                20,
                log_level.clone(),
            )),
            Some(duration),
        )),
    );

    let mut hex = Game::new(players);
    hex.set_duration(duration);
    hex.set_log_level(log_level);
    hex.play();

    /*

    let mut players = Vec::new();

    players.push(Rc::new(Player::new(
        "Random".to_string(),
        Rc::new(Random::new()),
        None,
    )));

    players.push(Rc::new(Player::new(
        "MiniMax".to_string(),
        Rc::new(MiniMax::new(Rc::new(Evaluation1::new()), 10, log_level.clone())),
        Some(duration),
    )));

    players.push(Rc::new(Player::new(
        "AlphaBeta".to_string(),
        Rc::new(AlphaBeta::new(Rc::new(Evaluation1::new()), 10, log_level.clone())),
        Some(duration),
    )));

    players.push(Rc::new(Player::new(
        "AlphaBeta2".to_string(),
        Rc::new(AlphaBeta2::new(Rc::new(Evaluation1::new()), 10, log_level.clone())),
        Some(duration),
    )));

    let mut tournament = Tournament::new();
    tournament.set_players(players);
    tournament.create_games(7, 10);
    tournament.play();
    tournament.print_results();
    */

    /*
    let mut players = Vec::new();

    players.push(Rc::new(Player::new(
        "AlphaBeta2".to_string(),
        Rc::new(AlphaBeta4::new(
            Rc::new(Evaluation1::new()),
            10,
            log_level.clone(),
        )),
        Some(duration),
    )));

    players.push(Rc::new(Player::new(
        "AlphaBeta4".to_string(),
        Rc::new(AlphaBeta4::new(
            Rc::new(Evaluation2::new()),
            10,
            log_level.clone(),
        )),
        Some(duration),
    )));
    let mut tournament = Tournament::new();
    tournament.set_players(players);
    tournament.create_games(5, 5);
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
