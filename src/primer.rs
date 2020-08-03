use crate::board::{Board, Cell};

#[derive(Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn prune(board: &mut Board, difficulty: Difficulty) -> u8 {
    let sequence = random_sequence();

    let holes = match difficulty {
        Difficulty::Easy => 41,
        Difficulty::Medium => 61,
        Difficulty::Hard => 81,
    };

    let mut removed = 0;
    let mut empty_cells = Vec::<Cell>::new();

    for cell in sequence.iter().map(Clone::clone).map(Cell::from_index) {
        if let Some(value) = board.get(cell) {
            if multiple_solutions(board, value, cell, &empty_cells) {
                board.set(cell, value);
            } else {
                board.clear(cell);
                empty_cells.push(cell);
                removed += 1;

                if removed >= holes {
                    break;
                }
            }
        }
    }

    removed
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

fn multiple_solutions(board: &Board, value: u8, cell: Cell, empty_cells: &[Cell]) -> bool {
    if empty_cells.is_empty() {
        return false;
    }
    crossbeam_utils::thread::scope(|s| {
        let results = (1..=2)
            .map(|i| {
                if i == value {
                    return None;
                }

                let mut cloned_board = *board;
                if cloned_board.set(cell, i) {
                    None
                } else {
                    Some(s.spawn(move |_| solvable(cloned_board, empty_cells, 0)))
                }
            })
            .collect::<Vec<_>>();

        for optional_result in results {
            if let Some(result) = optional_result {
                if result.join().expect("Failed to join threads") {
                    return true;
                }
            }
        }
        false
    })
    .expect("Failed to start scope")
}

fn solvable(mut board: Board, empty_cells: &[Cell], index: usize) -> bool {
    if index == empty_cells.len() {
        return true;
        // return board.list_inconsistencies().is_empty();
    }

    let cell = empty_cells[index];
    for value in 1..=9 {
        if board.set(cell, value) && board.list_inconsistencies().is_empty() {
            if solvable(board, empty_cells, index + 1) {
                board.clear(cell);
                return true;
            }
        }
    }

    board.clear(cell);
    false
}
