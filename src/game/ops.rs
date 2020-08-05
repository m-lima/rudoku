use super::{Cell, Game, Token};
use crate::index::RowIndexer;

pub fn consistent(game: &Game, cell: Cell, reference: Token) -> bool {
    if reference == Token::None {
        return true;
    }

    for current in RowIndexer::new(cell.row()) {
        if game.board[current.index()] == reference && current != cell {
            return false;
        }
    }

    // for current in crate::index::ColumnIndexer::new(cell.column()) {
    //     if game.board[current.index()] == reference && current != cell {
    //         return false;
    //     }
    // }

    {
        let row = cell.row();
        for (index, token) in game.columns[cell.column()].iter().enumerate() {
            if *token == reference && index != row {
                return false;
            }
        }
    }

    // for current in crate::index::SectorIndexer::new(cell.sector()) {
    //     if game.board[current.index()] == reference && current != cell {
    //         return false;
    //     }
    // }

    {
        let sector_index = cell.sector_index();
        for (index, token) in game.sectors[cell.sector()].iter().enumerate() {
            if *token == reference && index != sector_index {
                return false;
            }
        }
    }

    true
}

pub fn generate_solved() -> Game {
    let mut board = [Token::None; 81];

    board[0..9].copy_from_slice(&random_token_sequence()[..]);
    let game = Game::from(board);
    if let Some(solved) = solve(&game) {
        solved
    } else {
        unreachable!();
    }
}

pub fn solve(game: &Game) -> Option<Game> {
    let mut sequence = Vec::new();

    for cell in random_sequence().iter().map(Cell::from) {
        if game.get(cell) == Token::None {
            sequence.push(cell);
        }
    }

    solve_parallel(game, &sequence)
}

fn solve_parallel(game: &Game, sequence: &[Cell]) -> Option<Game> {
    if sequence.is_empty() {
        return Some(*game);
    }

    crossbeam_utils::thread::scope(|s| {
        let cell = sequence[0];
        let results = Token::list()
            .iter()
            .map(|token| {
                s.spawn(move |_| {
                    let mut thread_game = *game;
                    if consistent(&thread_game, cell, *token) {
                        thread_game.set_internal(cell, *token);
                        if solve_depth(&mut thread_game, sequence, 1) {
                            Some(thread_game)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        results
            .into_iter()
            .map(crossbeam::thread::ScopedJoinHandle::join)
            .filter_map(Result::ok)
            .flatten()
            .next()
    })
    .expect("Failed to start thread scope for solving game")
}

fn solve_depth(game: &mut Game, sequence: &[Cell], depth: usize) -> bool {
    if depth == sequence.len() {
        return true;
    }

    let cell = sequence[depth];
    for token in Token::list() {
        if consistent(game, cell, *token) {
            game.set_internal(cell, *token);
            if solve_depth(game, sequence, depth + 1) {
                return true;
            }
        }
    }

    game.set_internal(cell, Token::None);
    false
}

fn random_sequence() -> [u8; 81] {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    let mut indices = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    ];
    indices.shuffle(&mut rng);
    indices
}

fn random_token_sequence() -> [Token; 9] {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    let mut tokens = [Token::None; 9];
    tokens[0..9].copy_from_slice(Token::list());
    tokens.shuffle(&mut rng);
    tokens
}

#[cfg(test)]
pub fn consistent_board() -> super::Board {
    #[rustfmt::skip]
    let consistent = super::tokenize([
        1,2,3,4,5,6,7,8,9,
        4,5,6,7,8,9,1,2,3,
        7,8,9,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,1,
        5,6,7,8,9,1,2,3,4,
        8,9,1,2,3,4,5,6,7,
        3,4,5,6,7,8,9,1,2,
        6,7,8,9,1,2,3,4,5,
        9,1,2,3,4,5,6,7,8,
    ]);
    consistent
}

#[cfg(test)]
pub fn assert_consistent(game: &Game) {
    for cell in crate::index::BoardIndexer::new() {
        assert!(consistent(game, cell, game.get(cell)));
    }
}

#[cfg(test)]
mod tests {
    use super::super::tokenize;
    use super::{Cell, Game, Token};
    use crate::index::BoardIndexer;

    #[test]
    fn full_consistency() {
        let board = super::consistent_board();
        super::assert_consistent(&board.into());
    }

    #[test]
    fn row_inconsistency() {
        #[rustfmt::skip]
        let jig = Game::from(tokenize([
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            2,3,4,5,1,7,8,9,1,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
        ]));

        assert!(!super::consistent(&jig, Cell::new(3, 4), Token::One));
    }

    #[test]
    fn column_inconsistency() {
        #[rustfmt::skip]
        let jig = Game::from(tokenize([
            0,0,0,4,0,0,0,0,0,
            0,0,0,7,0,0,0,0,0,
            0,0,0,1,0,0,0,0,0,
            0,0,0,9,0,0,0,0,0,
            0,0,0,8,0,0,0,0,0,
            0,0,0,2,0,0,0,0,0,
            0,0,0,6,0,0,0,0,0,
            0,0,0,9,0,0,0,0,0,
            0,0,0,3,0,0,0,0,0,
        ]));

        assert!(!super::consistent(&jig, Cell::new(3, 3), Token::Nine));
    }

    #[test]
    fn sector_inconsistency() {
        #[rustfmt::skip]
        let jig = Game::from(tokenize([
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,5,6,7,0,0,0,
            0,0,0,8,2,1,0,0,0,
            0,0,0,2,3,4,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
        ]));

        assert!(!super::consistent(&jig, Cell::new(4, 4), Token::Two));
    }

    #[test]
    fn solve() {
        let mut game = Game::from(super::consistent_board());
        for cell in super::random_sequence().iter().take(10).map(Cell::from) {
            game.set_internal(cell, Token::None);
        }

        let solved = super::solve(&game);
        assert!(solved.is_some());

        let solved = solved.unwrap();
        for cell in BoardIndexer::new() {
            assert_ne!(solved.get(cell), Token::None);
            assert!(super::consistent(&game, cell, game.get(cell)));
        }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    extern crate test;

    use super::{Cell, Game, Token};
    use test::Bencher;

    #[bench]
    fn consistent(bench: &mut Bencher) {
        let game = Game::from(super::consistent_board());
        bench.iter(|| {
            super::assert_consistent(&game);
        });
    }

    #[bench]
    fn solve(bench: &mut Bencher) {
        let mut game = Game::from(super::consistent_board());
        for cell in super::random_sequence().iter().take(30).map(Cell::from) {
            game.set_internal(cell, Token::None);
        }

        bench.iter(|| {
            assert!(super::solve(&game).is_some());
        });
    }
}
