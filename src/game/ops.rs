use super::{Cell, Difficulty, Game, Token};

pub fn consistent(game: &Game, cell: Cell, reference: Token) -> bool {
    if reference == Token::None {
        return true;
    }

    let row = cell.row();
    let col = cell.column();
    let mut index = 0;
    for token in game.board[row * 9..(row + 1) * 9].iter() {
        if *token == reference && index != col {
            return false;
        }
        index += 1;
    }

    index = 0;
    for token in &game.columns[col] {
        if *token == reference && index != row {
            return false;
        }
        index += 1;
    }

    let sector_index = cell.sector_index();
    index = 0;
    for token in &game.sectors[cell.sector()] {
        if *token == reference && index != sector_index {
            return false;
        }
        index += 1;
    }

    true
}

pub fn generate_solved() -> Game {
    let mut board = [Token::None; 81];

    board[0..9].copy_from_slice(&random_token_sequence()[..]);
    let game = Game::from(board);
    if let Some(solved) = solve(&game, false) {
        solved
    } else {
        unreachable!();
    }
}

pub fn prune_per_gaps(game: &Game, max_difficulty: Difficulty) -> [Option<Game>; 3] {
    let mut current_game = *game;
    let mut pruned = [None; 3];
    let sequence = random_filled_sequence(game);
    let mut removed = 81 - sequence.len();

    let mut start_time = std::time::Instant::now();
    for cell in sequence {
        if can_remove(current_game, cell) {
            current_game.set_internal(cell, Token::None);
            removed += 1;
            eprintln!(
                "Removed: {}, Last: {}s",
                removed,
                start_time.elapsed().as_secs_f64()
            );
            start_time = std::time::Instant::now();

            if removed == Difficulty::Easy.to_gaps() {
                pruned[0] = Some(current_game);
            } else if removed == Difficulty::Medium.to_gaps() {
                pruned[1] = Some(current_game);
            }

            if removed == max_difficulty.to_gaps() {
                break;
            }
        }
    }

    if removed > Difficulty::Medium.to_gaps() {
        pruned[2] = Some(current_game);
    } else if removed > Difficulty::Easy.to_gaps() {
        pruned[1] = Some(current_game);
    } else if removed > 0 {
        pruned[0] = Some(current_game);
    }

    pruned
}

pub fn prune_per_duration(game: &Game, max_difficulty: Difficulty) -> [Option<Game>; 3] {
    let mut current_game = *game;
    let mut pruned = [None; 3];
    let sequence = random_filled_sequence(game);

    let start_time = std::time::Instant::now();
    let mut changed = false;
    for cell in sequence {
        if can_remove(current_game, cell) {
            current_game.set_internal(cell, Token::None);
            changed = true;
        }

        // TODO this is broken
        let elapsed = start_time.elapsed();
        if elapsed > Difficulty::Easy.to_duration() {
            if elapsed > Difficulty::Medium.to_duration() {
                pruned[1] = Some(current_game);
                changed = false;
            } else {
                pruned[0] = Some(current_game);
                changed = false;
            }
        }

        if start_time.elapsed() >= max_difficulty.to_duration() {
            break;
        }
    }

    let elapsed = start_time.elapsed();
    if elapsed > Difficulty::Medium.to_duration() {
        if changed {
            pruned[2] = Some(current_game);
        }
    } else if elapsed > Difficulty::Easy.to_duration() {
        if changed {
            pruned[1] = Some(current_game);
        }
    } else {
        if changed {
            pruned[0] = Some(current_game);
        }
    }

    pruned
}

fn can_remove(mut game: Game, cell: Cell) -> bool {
    let original = game.get(cell);

    for token in Token::list() {
        if *token != original && consistent(&game, cell, *token) {
            game.set_internal(cell, *token);
            if solve(&game, true).is_some() {
                return false;
            }
        }
    }

    true
}

pub fn solve(game: &Game, maybe_parallel: bool) -> Option<Game> {
    let sequence = random_empty_sequence(game);

    if maybe_parallel && sequence.len() > 40 {
        solve_parallel(game, &sequence)
    } else {
        let mut game_copy = *game;
        if solve_depth(&mut game_copy, &sequence, 0) {
            Some(game_copy)
        } else {
            None
        }
    }
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

fn random_filled_sequence(game: &Game) -> Vec<Cell> {
    let mut sequence = Vec::new();

    for cell in random_sequence().iter().map(Cell::from) {
        if game.get(cell) != Token::None {
            sequence.push(cell);
        }
    }
    sequence
}

fn random_empty_sequence(game: &Game) -> Vec<Cell> {
    let mut sequence = Vec::new();

    for cell in random_sequence().iter().map(Cell::from) {
        if game.get(cell) == Token::None {
            sequence.push(cell);
        }
    }
    sequence
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
        1, 2, 3, 4, 5, 6, 7, 8, 9,
        4, 5, 6, 7, 8, 9, 1, 2, 3,
        7, 8, 9, 1, 2, 3, 4, 5, 6,
        2, 3, 4, 5, 6, 7, 8, 9, 1,
        5, 6, 7, 8, 9, 1, 2, 3, 4,
        8, 9, 1, 2, 3, 4, 5, 6, 7,
        3, 4, 5, 6, 7, 8, 9, 1, 2,
        6, 7, 8, 9, 1, 2, 3, 4, 5,
        9, 1, 2, 3, 4, 5, 6, 7, 8,
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
    use crate::index::BoardIndexer;

    use super::{Cell, Game, Token};
    use super::super::tokenize;

    #[test]
    fn full_consistency() {
        let board = super::consistent_board();
        super::assert_consistent(&board.into());
    }

    #[test]
    fn row_inconsistency() {
        #[rustfmt::skip]
            let jig = Game::from(tokenize([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            2, 3, 4, 5, 1, 7, 8, 9, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]));

        assert!(!super::consistent(&jig, Cell::new(3, 4), Token::One));
    }

    #[test]
    fn column_inconsistency() {
        #[rustfmt::skip]
            let jig = Game::from(tokenize([
            0, 0, 0, 4, 0, 0, 0, 0, 0,
            0, 0, 0, 7, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 9, 0, 0, 0, 0, 0,
            0, 0, 0, 8, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0,
            0, 0, 0, 6, 0, 0, 0, 0, 0,
            0, 0, 0, 9, 0, 0, 0, 0, 0,
            0, 0, 0, 3, 0, 0, 0, 0, 0,
        ]));

        assert!(!super::consistent(&jig, Cell::new(3, 3), Token::Nine));
    }

    #[test]
    fn sector_inconsistency() {
        #[rustfmt::skip]
            let jig = Game::from(tokenize([
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 5, 6, 7, 0, 0, 0,
            0, 0, 0, 8, 2, 1, 0, 0, 0,
            0, 0, 0, 2, 3, 4, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]));

        assert!(!super::consistent(&jig, Cell::new(4, 4), Token::Two));
    }

    #[test]
    fn solve() {
        let mut game = Game::from(super::consistent_board());
        for cell in super::random_sequence().iter().take(10).map(Cell::from) {
            game.set_internal(cell, Token::None);
        }

        let solved = super::solve(&game, false);
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

    use test::Bencher;

    use super::{Cell, Game, Token};

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
            assert!(super::solve(&game, false).is_some());
        });
    }
}
