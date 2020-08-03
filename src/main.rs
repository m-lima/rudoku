#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]

mod board;
mod primer;

fn main() {
    let mut board = board::Board::new();
    let level = std::env::args()
        .nth(1)
        .map(|level| level.chars().filter(|c| c == &'+').count())
        .map_or(primer::Difficulty::Medium, |level| match level {
            0 | 2 => primer::Difficulty::Medium,
            1 => primer::Difficulty::Easy,
            _ => primer::Difficulty::Hard,
        });
    println!("{}", board);
    let removed = primer::prune(&mut board, level);
    println!("{}", board);
    println!("Removed {}", removed);
}
