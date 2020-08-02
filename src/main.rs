mod board;
mod prepare;

fn main() {
    let mut board = board::Board::new();
    let level = std::env::args()
        .skip(1)
        .next()
        .map(|level| level.chars().filter(|c| c == &'+').count())
        .map(|level| match level {
            0 | 2 => prepare::Difficulty::Medium,
            1 => prepare::Difficulty::Easy,
            _ => prepare::Difficulty::Hard,
        })
        .unwrap_or(prepare::Difficulty::Medium);
    println!("{}", board);
    let removed = prepare::prune(&mut board, level);
    println!("{}", board);
    println!("Removed {}", removed);
}
