use super::board::Board;
use super::{Cell, Token};

pub fn consistent(board: &Board, cell: Cell) -> bool {
    let reference = board.get(cell);

    if Token::None == reference {
        return true;
    }

    let mut found = false;
    for token in board.row(cell.row) {
        if token == reference as u8 {
            if found {
                return false;
            } else {
                found = true;
            }
        }
    }

    found = false;
    for token in board.column(cell.col) {
        if token == reference as u8 {
            if found {
                return false;
            } else {
                found = true;
            }
        }
    }

    found = false;
    for token in board.cluster(cell.row / 3 + cell.col / 3) {
        if token == reference as u8 {
            if found {
                return false;
            } else {
                found = true;
            }
        }
    }

    true
}

pub fn solve(original_board: &Board) -> Option<Board> {
    let mut board = *original_board;
    let mut sequence = Vec::new();

    for cell in random_sequence().iter().map(Cell::from) {
        if board.get(cell) == Token::None {
            sequence.push(cell);
        }
    }

    if solve_depth(&mut board, &sequence, 0) {
        Some(board)
    } else {
        None
    }
}

fn solve_depth(board: &mut Board, sequence: &[Cell], depth: usize) -> bool {
    if depth == sequence.len() {
        return true;
    }

    let cell = sequence[depth];
    for token in Token::iter() {
        board.set(cell, token);
        if !consistent(board, cell) {
            continue;
        }
        if solve_depth(board, sequence, depth + 1) {
            return true;
        }
    }
    board.set(cell, Token::None);
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

// fn prune(board: &mut Board, difficulty: Difficulty) -> u8 {
//     use std::io::Write;
//     let sequence = random_sequence();

//     let holes = match difficulty {
//         Difficulty::Easy => 41,
//         Difficulty::Medium => 61,
//         Difficulty::Hard => 81,
//     };

//     let mut removed = 0;
//     let mut empty_cells = Vec::<Cell>::new();

//     let _ = std::io::stderr().flush();
//     for (i, cell) in sequence.iter().map(Cell::from).enumerate() {
//         eprint!("\rRemoved: {}/{} ({}%)", removed, holes, (i * 100) / 81);
//         let _ = std::io::stderr().flush();

//         let token = board.get(cell);

//         if token == Token::None {
//             continue;
//         }

//         // if multiple_solutions(board, token, cell, &empty_cells) {
//         //     board.set(cell, token);
//         // } else {
//         board.set(cell, Token::None);
//         empty_cells.push(cell);
//         removed += 1;

//         if removed >= holes {
//             break;
//         }
//         // }
//     }
//     eprint!("\r");
//     let _ = std::io::stderr().flush();

//     removed
// }

// fn multiple_solutions(board: &Board, token: Token, cell: Cell, empty_cells: &[Cell]) -> bool {
//     if empty_cells.is_empty() {
//         return false;
//     }
//     crossbeam_utils::thread::scope(|s| {
//         let results = Token::iter()
//             .map(|i| {
//                 if i == token {
//                     return None;
//                 }

//                 let mut cloned_board = *board;
//                 if cloned_board.set(cell, i) {
//                     Some(s.spawn(move |_| solvable(cloned_board, empty_cells, 0)))
//                 } else {
//                     None
//                 }
//             })
//             .collect::<Vec<_>>();

//         for optional_result in results {
//             if let Some(result) = optional_result {
//                 if result.join().expect("Failed to join threads") {
//                     return true;
//                 }
//             }
//         }
//         false
//     })
//     .expect("Failed to start scope")
// }

// fn solvable(mut board: Board, empty_cells: &[Cell], index: usize) -> bool {
//     if index == empty_cells.len() {
//         return true;
//     }

//     let cell = empty_cells[index];
//     for token in Token::iter() {
//         if board.set(cell, token) && solvable(board, empty_cells, index + 1) {
//             board.set(cell, Token::None);
//             return true;
//         }
//     }

//     board.set(cell, Token::None);
//     false
// }

#[cfg(all(test, nightly))]
mod benches {
    extern crate test;
    use super::{Board, Cell};
    use test::Bencher;

    #[bench]
    fn consistent_orig(bench: &mut Bencher) {
        let board = Board::consistent();

        bench.iter(|| {
            for row in 0..9 {
                for col in 0..9 {
                    super::consistent(&board, Cell::new(row, col));
                }
            }
        });
    }
}
