mod ops;
mod transform;

use crate::index::{ColumnIndexer, SectorIndexer};

type Board = [Token; 81];

#[derive(Copy, Clone)]
pub struct Game {
    board: [Token; 81],
    columns: [[Token; 9]; 9],
    sectors: [[Token; 9]; 9],
}

impl Game {
    pub fn new_empty() -> Self {
        Self::from([Token::None; 81])
    }

    pub fn new_solved() -> Self {
        ops::generate_solved()
    }

    #[inline]
    pub fn get(&self, cell: Cell) -> Token {
        self.board[cell.index()]
    }

    #[must_use]
    pub fn set(&mut self, cell: Cell, token: Token) -> bool {
        self.set_internal(cell, token);
        ops::consistent(&self, cell, token)
    }

    fn set_internal(&mut self, cell: Cell, token: Token) {
        self.board[cell.index()] = token;
        self.columns[cell.column()][cell.row()] = token;
        self.sectors[cell.sector()][cell.sector_index()] = token;
    }

    pub fn solve(&self) -> Option<Self> {
        ops::solve(&self, false)
    }

    pub fn prune_per_gaps(&self, max_difficulty: Difficulty) -> [Option<Game>; 3] {
        ops::prune_per_gaps(self, max_difficulty)
    }

    pub fn prune_per_time(&self, max_difficulty: Difficulty) -> [Option<Game>; 3] {
        ops::prune_per_duration(self, max_difficulty)
    }
}

impl std::convert::From<Board> for Game {
    fn from(board: Board) -> Self {
        let mut columns = [[Token::None; 9]; 9];
        let mut sectors = [[Token::None; 9]; 9];

        for i in 0..9 {
            for (index, cell) in ColumnIndexer::new(i).enumerate() {
                columns[i][index] = board[cell.index()];
            }
            for (index, cell) in SectorIndexer::new(i).enumerate() {
                sectors[i][index] = board[cell.index()];
            }
        }
        Self {
            board,
            columns,
            sectors,
        }
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
            write!(fmt, "{:?},", self.board[i])?;
        }
        write!(fmt, "{:?}", self.board[80])
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn to_gaps(self) -> usize {
        match self {
            Difficulty::Easy => 40,
            Difficulty::Medium => 50,
            Difficulty::Hard => 60,
        }
    }

    fn to_duration(self) -> std::time::Duration {
        std::time::Duration::from_secs(match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => 30,
            Difficulty::Hard => 60,
        })
    }
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
    fn sector_index(self) -> usize {
        let row = self.0 / 9;
        let col = self.0 % 9;
        (row % 3) * 3 + (col % 3)
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
        write!(
            fmt,
            "[raw: {}, row: {}, column: {}, sector: {}, sector_index: {}]",
            self.0,
            self.row(),
            self.column(),
            self.sector(),
            self.sector_index()
        )
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
    use super::{ops, transform, Game};
    use crate::index::{BoardIndexer, ColumnIndexer, RowIndexer, SectorIndexer};

    #[test]
    fn game_from_array() {
        #[rustfmt::skip]
        let reference = ops::consistent_board();
        let game = Game::from(reference);

        for i in 0..9 {
            for cell in RowIndexer::new(i) {
                assert_eq!(game.get(cell), reference[cell.index()]);
            }

            for (index, cell) in ColumnIndexer::new(i).enumerate() {
                assert_eq!(game.columns[i][index], reference[cell.index()]);
            }

            for (index, cell) in SectorIndexer::new(i).enumerate() {
                assert_eq!(game.sectors[i][index], reference[cell.index()]);
            }
        }
    }

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
    fn cell_sector_index() {
        #[rustfmt::skip]
        let jig: [usize; 81] = [
            0,1,2,0,1,2,0,1,2,
            3,4,5,3,4,5,3,4,5,
            6,7,8,6,7,8,6,7,8,
            0,1,2,0,1,2,0,1,2,
            3,4,5,3,4,5,3,4,5,
            6,7,8,6,7,8,6,7,8,
            0,1,2,0,1,2,0,1,2,
            3,4,5,3,4,5,3,4,5,
            6,7,8,6,7,8,6,7,8,
        ];

        for cell in BoardIndexer::new() {
            assert_eq!(cell.sector_index(), jig[cell.index()]);
        }
    }

    #[test]
    fn transform_consistency() {
        let mut board = ops::consistent_board();
        transform::shift(&mut board, 2);
        ops::assert_consistent(&board.into());
        transform::rotate(&mut board);
        ops::assert_consistent(&board.into());
        transform::mirror_columns(&mut board);
        ops::assert_consistent(&board.into());
        transform::mirror_rows(&mut board);
        ops::assert_consistent(&board.into());
        transform::swap_columns(&mut board, 1, 1);
        ops::assert_consistent(&board.into());
        transform::swap_rows(&mut board, 1, 1);
        ops::assert_consistent(&board.into());
        transform::swap_column_sector(&mut board, 1);
        ops::assert_consistent(&board.into());
        transform::swap_row_sector(&mut board, 1);
        ops::assert_consistent(&board.into());
    }
}
