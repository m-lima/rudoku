#![deny(warnings, clippy::pedantic, rust_2018_idioms)]
#![cfg_attr(all(test, nightly), feature(test))]
#![allow(dead_code)]

mod game;
mod index;

fn main() {
    // let difficulty = std::env::args()
    //     .nth(1)
    //     .map(|level| level.chars().filter(|c| c == &'+').count())
    //     .map_or(game::Difficulty::Medium, |level| match level {
    //         0 | 2 => game::Difficulty::Medium,
    //         1 => game::Difficulty::Easy,
    //         _ => game::Difficulty::Hard,
    //     });

    let board = game::Game::new_solved();
    println!("{:?}", board);
    println!("{}", board);
}
