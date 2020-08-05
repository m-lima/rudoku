mod ops;
mod transform;

type Board = [Token; 81];

#[derive(Copy, Clone)]
pub struct Game(Board);

impl Game {
    pub fn new_empty() -> Self {
        Self([Token::None; 81])
    }

    pub fn new_solved() -> Self {
        Self(ops::generate_solved())
    }

    #[inline]
    pub fn get(&self, cell: Cell) -> Token {
        self.0[cell.index()]
    }

    #[inline]
    #[must_use]
    pub fn set(&mut self, cell: Cell, token: Token) -> bool {
        self.0[cell.index()] = token;
        ops::consistent(&self.0, cell)
    }

    pub fn list_inconsistencies(&self) -> Vec<Cell> {
        let mut inconsistencies = Vec::new();
        for index in 0..81 {
            let cell = Cell(index);
            if !ops::consistent(&self.0, cell) {
                inconsistencies.push(cell);
            }
        }
        inconsistencies
    }

    pub fn solve(&self) -> Option<Self> {
        ops::solve(&self.0).map(Self)
    }
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
            write!(fmt, "{:?},", self.0[i])?;
        }
        write!(fmt, "{:?}", self.0[80])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    pub fn list() -> &'static [Self] {
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

impl std::fmt::Debug for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", *self as u8)
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
fn tokenize(board: [u8; 81]) -> Board {
    let mut tokens = [Token::None; 81];
    for i in 0..81 {
        tokens[i] = Token::from(board[i]);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::{ops, transform};
    use crate::index::BoardIndexer;

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

        for cell in BoardIndexer::new() {
            assert_eq!(cell.sector(), jig[cell.index()]);
        }
    }

    #[test]
    fn transform_consistency() {
        let mut board = ops::consistent_board();
        transform::shift(&mut board, 2);
        ops::assert_consistent(&board);
        transform::rotate(&mut board);
        ops::assert_consistent(&board);
        transform::mirror_columns(&mut board);
        ops::assert_consistent(&board);
        transform::mirror_rows(&mut board);
        ops::assert_consistent(&board);
        transform::swap_columns(&mut board, 1, 1);
        ops::assert_consistent(&board);
        transform::swap_rows(&mut board, 1, 1);
        ops::assert_consistent(&board);
        transform::swap_column_sector(&mut board, 1);
        ops::assert_consistent(&board);
        transform::swap_row_sector(&mut board, 1);
        ops::assert_consistent(&board);
    }
}
