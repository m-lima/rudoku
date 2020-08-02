mod board;

fn main() {
    let board = board::Board::initialize();
    println!("{:?}", board);
    println!("Hello, world!");
}
