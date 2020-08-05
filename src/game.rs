type Board = [Token; 81];

#[derive(Copy, Clone)]
pub struct Game {
    board: Board,
}

impl Game {
    #[inline]
    pub fn get(&self, cell: Cell) -> Token {
        self.board[cell.index()]
    }

    #[inline]
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

    #[inline]
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

    #[inline]
    pub fn sector(self) -> usize {
        let row = self.0 / 9;
        let col = self.0 % 9;
        (row / 3) * 3 + col / 3
    }

    #[inline]
    pub(super) fn index(self) -> usize {
        self.0
    }

    #[inline]
    fn row(self) -> usize {
        self.0 / 9
    }

    #[inline]
    fn column(self) -> usize {
        self.0 % 9
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

impl std::fmt::Debug for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "[row: {}, column: {}]", self.row(), self.column())
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

mod transform {
    use super::{Board, Token};
    use crate::index::{ColumnIndexer, RowIndexer};

    fn shift(board: &mut Board, amount: u8) {
        for i in 0..81 {
            let token = board[i];
            if token != Token::None {
                board[i] = Token::from(token as u8 + amount % 10)
            }
        }
    }

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

    fn mirror_columns(board: &mut Board) {
        let other = *board;
        for i in 0..9 {
            let first = ColumnIndexer::new(i);
            let second = ColumnIndexer::new(8 - i);
            for (first, second) in first.zip(second) {
                board[first.index()] = other[second.index()];
            }
        }
    }

    fn mirror_rows(board: &mut Board) {
        let other = *board;
        for i in 0..9 {
            let first = RowIndexer::new(i);
            let second = RowIndexer::new(8 - i);
            for (first, second) in first.zip(second) {
                board[first.index()] = other[second.index()];
            }
        }
    }

    fn swap_columns(board: &mut Board, cluster_column: usize, pivot: usize) {
        let other = *board;
        let col1 = ((pivot + 1) % 3) + cluster_column * 3;
        let col2 = ((pivot + 2) % 3) + cluster_column * 3;

        let first = ColumnIndexer::new(col1);
        let second = ColumnIndexer::new(col2);
        for (first, second) in first.zip(second) {
            board[first.index()] = other[second.index()];
            board[second.index()] = other[first.index()];
        }
    }

    fn swap_rows(board: &mut Board, cluster_row: usize, pivot: usize) {
        let other = *board;
        let row1 = (((pivot + 1) % 3) + cluster_row * 3) * 9;
        let row2 = (((pivot + 2) % 3) + cluster_row * 3) * 9;

        board[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        board[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    }

    fn swap_column_cluster(board: &mut Board, pivot: usize) {
        let other = *board;
        let col1 = ((pivot + 1) % 3) * 3;
        let col2 = ((pivot + 1) % 3) * 3;

        let first = ColumnIndexer::new(col1);
        let second = ColumnIndexer::new(col2);
        for (first, second) in first.zip(second) {
            board[first.index()] = other[second.index()];
            board[first.index() + 1] = other[second.index() + 1];
            board[first.index() + 1] = other[second.index() + 2];

            board[second.index()] = other[first.index()];
            board[second.index() + 1] = other[first.index() + 1];
            board[second.index() + 2] = other[first.index() + 2];
        }
    }

    fn swap_row_cluster(board: &mut Board, pivot: usize) {
        let other = *board;
        let row1 = (((pivot + 1) % 3) * 3) * 9;
        let row2 = (((pivot + 1) % 3) * 3) * 9;

        board[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        board[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        board[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        board[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
        board[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
        board[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    }
}
