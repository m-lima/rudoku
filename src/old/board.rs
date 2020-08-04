// pub mod shuffle;
use super::{Cell, Token};

#[derive(Copy, Clone)]
pub struct Builder([u8; 81]);

#[derive(Copy, Clone)]
pub struct Board {
    row_major: [[Token; 9]; 9],
    col_major: [[Token; 9]; 9],
    sec_major: [[Token; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            row_major: [[Token::None; 9]; 9],
            col_major: [[Token::None; 9]; 9],
            sec_major: [[Token::None; 9]; 9],
        }
    }

    pub fn get(&self, cell: Cell) -> Token {
        self.row_major[cell.row()][cell.col()]
    }

    pub fn set(&mut self, cell: Cell, token: Token) {
        self.row_major[cell.row()][cell.col()] = token;
        self.col_major[cell.col()][cell.row()] = token;
        self.sec_major[cell.sec()][cell.idx()] = token;
    }

    pub fn row(&self, row: usize) -> &[Token; 9] {
        assert!(row < 9, "Row index out of bounds: {}", row);
        &self.row_major[row]
    }

    pub fn column(&self, column: usize) -> &[Token; 9] {
        assert!(column < 9, "Column index out of bounds: {}", column);
        &self.col_major[column]
    }

    pub fn sector(&self, sector: usize) -> &[Token; 9] {
        assert!(sector < 9, "Sector index out of bounds: {}", sector);
        &self.sec_major[sector]
    }
}

impl std::fmt::Display for Board {
    // Allowed because it is more readable
    #[allow(clippy::non_ascii_literal)]
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "┏━━━━━┯━━━━━┯━━━━━┓")?;
        for row in 0..9 {
            write!(fmt, "┃")?;
            for col in 0..8 {
                if col % 3 == 2 {
                    write!(fmt, "{}│", self.row_major[row][col])?;
                } else {
                    write!(fmt, "{} ", self.row_major[row][col])?;
                }
            }
            writeln!(fmt, "{}┃", self.row_major[row][8])?;
            if row < 8 && row % 3 == 2 {
                writeln!(fmt, "┠─────┼─────┼─────┨")?;
            }
        }
        writeln!(fmt, "┗━━━━━┷━━━━━┷━━━━━┛")
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..8 {
            for col in 0..9 {
                write!(fmt, "{},", self.row_major[row][col])?;
            }
        }
        for col in 0..8 {
            write!(fmt, "{},", self.row_major[8][col])?;
        }
        write!(fmt, "{}", self.row_major[8][8])
    }
}

// // Allowed because this is a test
// #[allow(clippy::cast_possible_truncation)]
// #[cfg(test)]
// mod tests {
//     use super::{Board, Cell, IntoClusterIterator, IntoColumnIterator, IntoRowIterator, Token};

//     #[test]
//     fn get() {
//         let board = Board::consistent();

//         for row in 0..9 {
//             for col in 0..9 {
//                 assert_eq!(
//                     board.get(Cell { row, col }) as u8,
//                     board.0[usize::from(row * 9 + col)]
//                 );
//             }
//         }
//     }

//     #[test]
//     fn set() {
//         let mut board = Board::consistent();
//         board.set(Cell { row: 4, col: 5 }, Token::One);

//         for row in 0..9 {
//             for col in 0..9 {
//                 if row == 4 && col == 5 {
//                     assert_eq!(board.get(Cell { row, col }), Token::One);
//                 } else {
//                     assert_eq!(
//                         board.get(Cell { row, col }) as u8,
//                         board.0[usize::from(row * 9 + col)]
//                     );
//                 }
//             }
//         }
//     }

//     #[test]
//     fn reverse() {
//         let mut board = Board::sequential();
//         board.reverse();

//         for i in 0..81 {
//             assert_eq!(board.0[usize::from(i)], 80 - i);
//         }
//     }

//     #[test]
//     fn rotate() {
//         let mut board = Board::sequential();
//         board.rotate();

//         for row in 0..9 {
//             for col in 0..9 {
//                 assert_eq!(board.0[usize::from(row * 9 + col)], col * 9 + row);
//             }
//         }
//     }

//     #[test]
//     fn mirror_columns() {
//         let mut board = Board::sequential();
//         board.mirror_columns();

//         for row in 0..9 {
//             for col in 0..9 {
//                 assert_eq!(board.0[usize::from(row * 9 + col)], row * 9 + (8 - col));
//             }
//         }
//     }

//     #[test]
//     fn mirror_rows() {
//         let mut board = Board::sequential();
//         board.mirror_rows();

//         for row in 0..9 {
//             for col in 0..9 {
//                 assert_eq!(board.0[usize::from(row * 9 + col)], (8 - row) * 9 + col);
//             }
//         }
//     }

//     #[test]
//     fn swap_columns() {
//         let mut board = Board::consistent();
//         board.swap_columns(0, 2);
//         let expected = Board::consistent();

//         for row in 0..9 {
//             for col in 0..9 {
//                 if col == 0 {
//                     assert_eq!(
//                         board.get(Cell { row, col }),
//                         expected.get(Cell { row, col: col + 1 })
//                     );
//                 } else if col == 1 {
//                     assert_eq!(
//                         board.get(Cell { row, col }),
//                         expected.get(Cell { row, col: col - 1 })
//                     );
//                 } else {
//                     assert_eq!(
//                         board.get(Cell { row, col }),
//                         expected.get(Cell { row, col })
//                     );
//                 }
//             }
//         }
//     }

//     #[test]
//     fn swap_rows() {
//         let mut board = Board::consistent();
//         board.swap_rows(0, 2);
//         let expected = Board::consistent();

//         for row in 0..9 {
//             for col in 0..9 {
//                 if row == 0 {
//                     assert_eq!(
//                         board.get(Cell { row, col }),
//                         expected.get(Cell { row: row + 1, col })
//                     );
//                 } else if row == 1 {
//                     assert_eq!(
//                         board.get(Cell { row, col }),
//                         expected.get(Cell { row: row - 1, col })
//                     );
//                 } else {
//                     assert_eq!(
//                         board.get(Cell { row, col }),
//                         expected.get(Cell { row, col })
//                     );
//                 }
//             }
//         }
//     }

//     #[test]
//     fn swap_column_cluster() {
//         let mut board = Board::consistent();
//         board.swap_column_cluster(2);
//         board.swap_column_cluster(1);
//         let expected = Board::consistent();

//         for row in 0..9 {
//             for col_index in 0..9 {
//                 let col = (col_index + 1) % 3;
//                 assert_eq!(
//                     board.get(Cell { row, col }),
//                     expected.get(Cell { row, col })
//                 );
//             }
//         }
//     }

//     #[test]
//     fn swap_row_cluster() {
//         let mut board = Board::consistent();
//         board.swap_row_cluster(2);
//         board.swap_row_cluster(1);
//         let expected = Board::consistent();

//         for row_index in 0..9 {
//             for col in 0..9 {
//                 let row = (row_index + 1) % 3;
//                 assert_eq!(
//                     board.get(Cell { row, col }),
//                     expected.get(Cell { row, col })
//                 );
//             }
//         }
//     }

//     #[test]
//     fn shift() {
//         let mut board = Board::sequential();
//         board.shift(1);

//         for i in 0..81 {
//             assert_eq!(board.0[usize::from(i)], (i + 1) % 9 + 1);
//         }
//     }

//     #[test]
//     fn row_iterator() {
//         let board = Board::sequential();

//         for i in 0..9 {
//             let iter = board.0.row(i);
//             for (index, token) in iter.enumerate() {
//                 assert_eq!(token, i * 9 + index as u8);
//             }
//         }
//     }

//     #[test]
//     fn column_iterator() {
//         let board = Board::sequential();

//         for i in 0..9 {
//             let iter = board.0.column(i);
//             for (index, token) in iter.enumerate() {
//                 assert_eq!(token, index as u8 * 9 + i);
//             }
//         }
//     }

//     #[test]
//     fn cluster_iterator() {
//         let board = Board::sequential();

//         for i in 0..9 {
//             let base = (27 * (i / 3)) + 3 * (i % 3);
//             let iter = board.0.cluster(i);
//             for (index, token) in iter.enumerate() {
//                 let byte_index = index as u8;
//                 let expected = base + (9 * (byte_index / 3)) + (byte_index % 3);
//                 assert_eq!(token, expected);
//             }
//         }
//     }

//     #[test]
//     fn transforms_are_consistent() {
//         use rand::Rng;

//         let mut game = super::super::Game {
//             board: Board::consistent(),
//         };

//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.reverse();
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.rotate();
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.mirror_columns();
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.mirror_rows();
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.swap_columns(2, 1);
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.swap_rows(1, 2);
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.swap_column_cluster(1);
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.swap_row_cluster(1);
//         assert_eq!(game.list_inconsistencies().len(), 0);
//         game.board.shift(1);
//         assert_eq!(game.list_inconsistencies().len(), 0);

//         let lucky_index = rand::thread_rng().gen::<u8>() % 81;
//         let lucky_cell = Cell::from(lucky_index);
//         let cell = game.board.get(lucky_cell) as u8;
//         game.board.0[usize::from(lucky_index)] = ((cell + 1) % 9) + 1;
//         assert!(!game.list_inconsistencies().is_empty());
//     }
// }
