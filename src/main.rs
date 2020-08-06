#![deny(warnings, clippy::pedantic, rust_2018_idioms)]
#![cfg_attr(all(test, nightly), feature(test))]
#![allow(dead_code)]

mod game;
mod index;

fn main() {
    let options = std::env::args()
        .skip(1)
        .take(2)
        .fold((None, None), |acc, curr| {
            if acc.0.is_none() {
                (Some(curr), None)
            } else {
                (acc.0, Some(curr))
            }
        });

    let difficulty = options
        .1
        .map(|level| level.chars().filter(|c| c == &'+').count())
        .map_or(game::Difficulty::Easy, |level| match level {
            0 | 1 => game::Difficulty::Easy,
            2 => game::Difficulty::Medium,
            _ => game::Difficulty::Hard,
        });

    let game = game::Game::new_solved();
    eprintln!("{}", game);
    println!("Solved: [{:?}]", game);
    let pruned = match options.0.unwrap_or(String::from("t")).as_str() {
        "g" | "gap" | "gaps" => game.prune_per_gaps(difficulty),
        "t" | "time" => game.prune_per_time(difficulty),
        _ => panic!("Unkown option"),
    };
    if let Some(game) = pruned[0] {
        println!("Easy:   [{:?}]", game)
    }
    if let Some(game) = pruned[1] {
        println!("Medium: [{:?}]", game)
    }
    if let Some(game) = pruned[2] {
        println!("Hard:   [{:?}]", game)
    }
}
