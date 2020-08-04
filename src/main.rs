#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]
// TODO remove
#![allow(dead_code)]
#![cfg_attr(all(test, nightly), feature(test))]

mod game;

fn main() {
    // let difficulty = std::env::args()
    //     .nth(1)
    //     .map(|level| level.chars().filter(|c| c == &'+').count())
    //     .map_or(game::Difficulty::Medium, |level| match level {
    //         0 | 2 => game::Difficulty::Medium,
    //         1 => game::Difficulty::Easy,
    //         _ => game::Difficulty::Hard,
    //     });
    // let game = game::Game::new(difficulty);
    // println!("{}", game.board());
    // println!("{:?}", game.board());
}
