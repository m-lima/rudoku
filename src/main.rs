mod board;
mod primer;

fn main() {
    let mut board = board::Board::new();
    let level = std::env::args()
        .nth(1)
        .map(|level| level.chars().filter(|c| c == &'+').count())
        .map(|level| match level {
            0 | 2 => primer::Difficulty::Medium,
            1 => primer::Difficulty::Easy,
            _ => primer::Difficulty::Hard,
        })
        .unwrap_or(primer::Difficulty::Medium);
    println!("{}", board);
    let removed = primer::prune(&mut board, level);
    println!("{}", board);
    println!("Removed {}", removed);
}
