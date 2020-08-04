type Board = [Token; 81];

#[derive(Copy, Clone)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn get(&self, cell: Cell) -> Token {
        self.board[cell.index()]
    }

    pub fn set(&mut self, cell: Cell, token: Token) -> bool {
        self.board[cell.index()] = token;
        true
        // ops::consistent(self.board, cell)
    }

    // pub fn list_inconsistencies(&self) -> Vec<Cell> {
    //     let mut inconsistencies = Vec::new();
    //     for index in 0..81 {
    //         let cell = Cell(index);
    //         if !ops::consistent(&self.board, cell) {
    //             inconsistencies.push(cell);
    //         }
    //     }
    //     inconsistencies
    // }
}

impl std::fmt::Display for Game {
    // Allowed because it is more readable
    #[allow(clippy::non_ascii_literal)]
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "┏━━━━━┯━━━━━┯━━━━━┓")?;
        for row in 0..9 {
            write!(fmt, "┃")?;
            for col in 0..8 {
                let cell = Cell::new(row, col);
                if col % 3 == 2 {
                    write!(fmt, "{}│", self.get(cell))?;
                } else {
                    write!(fmt, "{} ", self.get(cell))?;
                }
            }
            writeln!(fmt, "{}┃", self.get(Cell::new(row, 8)))?;

            if row < 8 && row % 3 == 2 {
                writeln!(fmt, "┠─────┼─────┼─────┨")?;
            }
        }
        writeln!(fmt, "┗━━━━━┷━━━━━┷━━━━━┛")
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..80 {
            write!(fmt, "{},", self.board[i])?;
        }
        write!(fmt, "{}", self.board[80])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Token {
    None = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Token {
    const TOKENS: [Self; 10] = [
        Token::None,
        Token::One,
        Token::Two,
        Token::Three,
        Token::Four,
        Token::Five,
        Token::Six,
        Token::Seven,
        Token::Eight,
        Token::Nine,
    ];

    pub fn iter() -> &'static [Self] {
        &Self::TOKENS[1..10]
    }
}

impl std::convert::From<u8> for Token {
    fn from(token: u8) -> Self {
        Self::TOKENS[usize::from(token)]
    }
}

impl std::convert::From<&u8> for Token {
    fn from(token: &u8) -> Self {
        Self::from(*token)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Token::None {
            write!(fmt, " ")
        } else {
            write!(fmt, "{}", *self as u8)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cell(usize);

impl Cell {
    #[inline]
    pub fn new(row: u8, column: u8) -> Self {
        assert!(
            row < 9 && column < 9,
            "Cell aout of bounds (row: {}, column: {})",
            row,
            column
        );
        Self(usize::from(row * 9 + column))
    }

    pub fn sector(self) -> usize {
        let row = self.0 / 9;
        let col = self.0 % 9;
        (row / 3) * 3 + col / 3
    }

    fn index(self) -> usize {
        self.0
    }
}

impl std::convert::From<usize> for Cell {
    fn from(index: usize) -> Self {
        assert!(index < 81, "Index out of bounds: {}", index);
        Self(index)
    }
}

impl std::convert::From<&u8> for Cell {
    fn from(index: &u8) -> Self {
        Self::from(usize::from(*index))
    }
}

impl std::convert::From<u8> for Cell {
    fn from(index: u8) -> Self {
        Self::from(usize::from(index))
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn cell_sector() {
        #[rustfmt::skip]
        let jig: [usize; 81] = [
            0,0,0,1,1,1,2,2,2,
            0,0,0,1,1,1,2,2,2,
            0,0,0,1,1,1,2,2,2,
            3,3,3,4,4,4,5,5,5,
            3,3,3,4,4,4,5,5,5,
            3,3,3,4,4,4,5,5,5,
            6,6,6,7,7,7,8,8,8,
            6,6,6,7,7,7,8,8,8,
            6,6,6,7,7,7,8,8,8,
        ];

        for i in 0..81 {
            let cell = Cell(i);
            assert_eq!(cell.sector(), jig[cell.index()]);
        }
    }
}

mod iter {
    use super::Cell;

    pub struct RowIndexer {
        index: usize,
        end: usize,
    }

    impl RowIndexer {
        pub fn new(row: usize) -> Self {
            assert!(row < 9, "Row index out of bounds: {}", row);
            let index = row * 9;
            Self {
                index,
                end: index + 9,
            }
        }
    }

    impl std::iter::Iterator for RowIndexer {
        type Item = Cell;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.end {
                let cell = Some(Cell::from(self.index));
                self.index += 1;
                cell
            } else {
                None
            }
        }
    }

    pub struct ColumnIndexer {
        index: usize,
    }

    impl ColumnIndexer {
        pub fn new(column: usize) -> Self {
            assert!(column < 9, "Column index out of bounds: {}", column);
            Self { index: column }
        }
    }

    impl std::iter::Iterator for ColumnIndexer {
        type Item = Cell;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < 81 {
                let cell = Some(Cell::from(self.index));
                self.index += 9;
                cell
            } else {
                None
            }
        }
    }

    pub struct SectorIndexer {
        index: usize,
        wall: usize,
        end: usize,
    }

    impl SectorIndexer {
        pub fn new(sector: usize) -> Self {
            assert!(sector < 9, "Sector index out of bounds: {}", sector);
            let index = (sector / 3) * 27 + (sector % 3) * 3;
            Self {
                index,
                wall: index + 3,
                end: index + 9 + 9 + 3,
            }
        }
    }

    impl std::iter::Iterator for SectorIndexer {
        type Item = Cell;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.end {
                let cell = Some(Cell::from(self.index));
                self.index += 1;
                if self.index == self.wall {
                    self.index += 6;
                    self.wall += 9;
                }
                cell
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{ColumnIndexer, RowIndexer, SectorIndexer};

        #[test]
        fn row_low() {
            #[rustfmt::skip]
            let jig: [usize; 81] = [
                0,1,2,3,4,5,6,7,8,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                ];

            let iter = RowIndexer::new(0);
            for (index, cell) in iter.enumerate() {
                assert_eq!(index, jig[cell.index()]);
            }
        }

        #[test]
        fn row_high() {
            #[rustfmt::skip]
            let jig: [usize; 81] = [
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,1,2,3,4,5,6,7,8,
                ];

            let iter = RowIndexer::new(8);
            for (index, cell) in iter.enumerate() {
                assert_eq!(index, jig[cell.index()]);
            }
        }

        #[test]
        fn column_low() {
            #[rustfmt::skip]
            let jig: [usize; 81] = [
                0,0,0,0,0,0,0,0,0,
                1,0,0,0,0,0,0,0,0,
                2,0,0,0,0,0,0,0,0,
                3,0,0,0,0,0,0,0,0,
                4,0,0,0,0,0,0,0,0,
                5,0,0,0,0,0,0,0,0,
                6,0,0,0,0,0,0,0,0,
                7,0,0,0,0,0,0,0,0,
                8,0,0,0,0,0,0,0,0,
                ];

            let iter = ColumnIndexer::new(0);
            for (index, cell) in iter.enumerate() {
                assert_eq!(index, jig[cell.index()]);
            }
        }

        #[test]
        fn column_high() {
            #[rustfmt::skip]
            let jig: [usize; 81] = [
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,1,
                0,0,0,0,0,0,0,0,2,
                0,0,0,0,0,0,0,0,3,
                0,0,0,0,0,0,0,0,4,
                0,0,0,0,0,0,0,0,5,
                0,0,0,0,0,0,0,0,6,
                0,0,0,0,0,0,0,0,7,
                0,0,0,0,0,0,0,0,8,
                ];

            let iter = ColumnIndexer::new(8);
            for (index, cell) in iter.enumerate() {
                assert_eq!(index, jig[cell.index()]);
            }
        }

        #[test]
        fn sector_low() {
            #[rustfmt::skip]
            let jig: [usize; 81] = [
                0,1,2,0,0,0,0,0,0,
                3,4,5,0,0,0,0,0,0,
                6,7,8,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                ];

            let iter = SectorIndexer::new(0);
            for (index, cell) in iter.enumerate() {
                assert_eq!(index, jig[cell.index()]);
            }
        }

        #[test]
        fn sector_high() {
            #[rustfmt::skip]
            let jig: [usize; 81] = [
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,1,2,
                0,0,0,0,0,0,3,4,5,
                0,0,0,0,0,0,6,7,8,
                ];

            let iter = SectorIndexer::new(8);
            for (index, cell) in iter.enumerate() {
                assert_eq!(index, jig[cell.index()]);
            }
        }
    }
}

mod transform {
    use super::iter::{ColumnIndexer, RowIndexer};
    use super::Board;

    fn rotate(board: &mut Board) {
        let other = *board;
        for i in 0..9 {
            let col = ColumnIndexer::new(i);
            let row = RowIndexer::new(i);
            for (col, row) in col.zip(row) {
                board[col.index()] = other[row.index()];
            }
        }
    }

    // fn mirror_columns(&mut self) {
    //     let other = self.0;
    //     for i in 0..9 {
    //         for (index, token) in other.column(i).enumerate() {
    //             self.0[index * 9 + (8 - usize::from(i))] = token;
    //         }
    //     }
    // }

    // fn mirror_rows(&mut self) {
    //     let other = self.0;
    //     for i in 0..9 {
    //         for (index, token) in other.row(i).enumerate() {
    //             self.0[(8 - usize::from(i)) * 9 + index] = token;
    //         }
    //     }
    // }

    // fn swap_columns(&mut self, cluster_column: u8, pivot: u8) {
    //     let other = self.0;
    //     let col1 = usize::from(((pivot + 1) % 3) + cluster_column * 3);
    //     let col2 = usize::from(((pivot + 2) % 3) + cluster_column * 3);

    //     for row in 0..9 {
    //         let row_ref = row * 9;
    //         self.0[row_ref + col1] = other[row_ref + col2];
    //         self.0[row_ref + col2] = other[row_ref + col1];
    //     }
    // }

    // fn swap_rows(&mut self, cluster_row: u8, pivot: u8) {
    //     let other = self.0;
    //     let row1 = usize::from(((pivot + 1) % 3) + cluster_row * 3) * 9;
    //     let row2 = usize::from(((pivot + 2) % 3) + cluster_row * 3) * 9;

    //     self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
    //     self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    // }

    // fn swap_column_cluster(&mut self, pivot: u8) {
    //     let other = self.0;
    //     let col1 = usize::from(((pivot + 1) % 3) * 3);
    //     let col2 = usize::from(((pivot + 1) % 3) * 3);

    //     for row in 0..9 {
    //         let row_ref = row * 9;
    //         self.0[row_ref + col1] = other[row_ref + col2];
    //         self.0[row_ref + col1 + 1] = other[row_ref + col2 + 1];
    //         self.0[row_ref + col1 + 2] = other[row_ref + col2 + 2];

    //         self.0[row_ref + col2] = other[row_ref + col1];
    //         self.0[row_ref + col2 + 1] = other[row_ref + col1 + 1];
    //         self.0[row_ref + col2 + 2] = other[row_ref + col1 + 2];
    //     }
    // }

    // fn swap_row_cluster(&mut self, pivot: u8) {
    //     let other = self.0;
    //     let row1 = usize::from(((pivot + 1) % 3) * 3) * 9;
    //     let row2 = usize::from(((pivot + 1) % 3) * 3) * 9;

    //     self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
    //     self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
    //     self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
    //     self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    //     self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    //     self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    // }

    // fn shift(&mut self, amount: u8) {
    //     for token in self.0.iter_mut() {
    //         *token = ((*token + amount) % 9) + 1;
    //     }
    // }
}
