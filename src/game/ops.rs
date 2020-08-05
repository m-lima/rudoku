use super::{Board, Cell, Token};
use crate::index::{ColumnIndexer, RowIndexer, SectorIndexer};

// TODO replace [Token; 81] with `Game`

pub fn consistent(board: &Board, cell: Cell) -> bool {
    let reference = board[cell.index()];

    if reference == Token::None {
        return true;
    }

    for current in RowIndexer::new(cell.row()) {
        if current == cell {
            continue;
        }

        if board[current.index()] == reference {
            return false;
        }
    }

    for current in ColumnIndexer::new(cell.column()) {
        if current == cell {
            continue;
        }

        if board[current.index()] == reference {
            return false;
        }
    }

    for current in SectorIndexer::new(cell.sector()) {
        if current == cell {
            continue;
        }

        if board[current.index()] == reference {
            return false;
        }
    }

    true
}

pub fn solve(board: &Board) -> Option<Board> {
    let mut board_copy = *board;
    let mut sequence = Vec::new();

    for cell in random_sequence().iter().map(Cell::from) {
        if board[cell.index()] == Token::None {
            sequence.push(cell);
        }
    }

    if solve_depth(&mut board_copy, &sequence, 0) {
        Some(board_copy)
    } else {
        None
    }
}

pub fn generate_solved() -> Board {
    let mut board = [Token::None; 81];

    board[0..9].copy_from_slice(&random_token_sequence()[..]);
    if let Some(solved) = solve(&board) {
        solved
    } else {
        unreachable!();
    }
}

fn solve_depth(board: &mut Board, sequence: &[Cell], depth: usize) -> bool {
    if depth == sequence.len() {
        return true;
    }

    let cell = sequence[depth];
    for token in Token::iter() {
        board[cell.index()] = *token;

        if !consistent(board, cell) {
            continue;
        }

        if solve_depth(board, sequence, depth + 1) {
            return true;
        }
    }

    board[cell.index()] = Token::None;
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
    tokens[0..9].copy_from_slice(Token::iter());
    tokens.shuffle(&mut rng);
    tokens
}

#[cfg(test)]
pub fn consistent_board() -> Board {
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
pub fn assert_consistent(board: &Board) {
    for cell in crate::index::BoardIndexer::new() {
        assert!(consistent(board, cell));
    }
}

#[cfg(test)]
mod tests {
    use super::super::tokenize;
    use super::{Cell, Token};
    use crate::index::BoardIndexer;

    #[test]
    fn full_consistency() {
        let board = super::consistent_board();
        super::assert_consistent(&board);
    }

    #[test]
    fn row_inconsistency() {
        #[rustfmt::skip]
        let jig = tokenize([
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            2,3,4,5,1,7,8,9,1,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
        ]);

        assert!(!super::consistent(&jig, Cell::new(3, 4)));
    }

    #[test]
    fn column_inconsistency() {
        #[rustfmt::skip]
        let jig = tokenize([
            0,0,0,4,0,0,0,0,0,
            0,0,0,7,0,0,0,0,0,
            0,0,0,1,0,0,0,0,0,
            0,0,0,9,0,0,0,0,0,
            0,0,0,8,0,0,0,0,0,
            0,0,0,2,0,0,0,0,0,
            0,0,0,6,0,0,0,0,0,
            0,0,0,9,0,0,0,0,0,
            0,0,0,3,0,0,0,0,0,
        ]);

        assert!(!super::consistent(&jig, Cell::new(3, 3)));
    }

    #[test]
    fn sector_inconsistency() {
        #[rustfmt::skip]
        let jig = tokenize([
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,5,6,7,0,0,0,
            0,0,0,8,2,1,0,0,0,
            0,0,0,2,3,4,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
        ]);

        assert!(!super::consistent(&jig, Cell::new(4, 4)));
    }

    #[test]
    fn solve() {
        let mut board = super::consistent_board();
        for cell in super::random_sequence().iter().take(10).map(Cell::from) {
            board[cell.index()] = Token::None;
        }

        let solved = super::solve(&board);
        assert!(solved.is_some());

        let solved = solved.unwrap();
        for cell in BoardIndexer::new() {
            assert_ne!(solved[cell.index()], Token::None);
            assert!(super::consistent(&board, cell));
        }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    extern crate test;

    use super::{Cell, Token};
    use test::Bencher;

    #[bench]
    fn consistent(bench: &mut Bencher) {
        let board = super::consistent_board();
        bench.iter(|| {
            super::assert_consistent(&board);
        });
    }

    #[bench]
    fn solve(bench: &mut Bencher) {
        let mut board = super::consistent_board();
        for cell in super::random_sequence().iter().take(30).map(Cell::from) {
            board[cell.index()] = Token::None;
        }

        bench.iter(|| {
            assert!(super::solve(&board).is_some());
        });
    }
}
