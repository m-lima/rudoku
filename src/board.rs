#[derive(Copy, Clone)]
pub struct Board([u8; 81]);

impl Board {
    fn initialize_base() -> Self {
        Self([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7, 8, 9, 1, 2, 3, 7, 8, 9, 1, 2, 3, 4, 5, 6, 2, 3,
            4, 5, 6, 7, 8, 9, 1, 5, 6, 7, 8, 9, 1, 2, 3, 4, 8, 9, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6,
            7, 8, 9, 1, 2, 6, 7, 8, 9, 1, 2, 3, 4, 5, 9, 1, 2, 3, 4, 5, 6, 7, 8,
        ])
    }

    pub fn new() -> Self {
        use rand::Rng;
        let mut board = Self::initialize_base();
        let mut rng = rand::thread_rng();
        for _ in 0..128 {
            match rng.gen::<u8>() % 9 {
                0 => board.reverse(),
                1 => board.rotate(),
                2 => board.mirror_columns(),
                3 => board.mirror_rows(),
                4 => board.swap_columns(rng.gen::<u8>() % 3, rng.gen::<u8>() % 3),
                5 => board.swap_rows(rng.gen::<u8>() % 3, rng.gen::<u8>() % 3),
                6 => board.swap_column_cluster(rng.gen::<u8>() % 3),
                7 => board.swap_row_cluster(rng.gen::<u8>() % 3),
                8 => board.shift(rng.gen::<u8>() % 7),
                _ => unreachable!(),
            }
        }
        assert!(board.list_inconsistencies().is_empty());
        board
    }

    #[inline]
    pub fn get(&self, cell: Cell) -> Token {
        Token::from(self.0[cell.as_linear()])
    }

    pub fn set(&mut self, cell: Cell, token: Token) -> bool {
        self.0[cell.as_linear()] = token as u8;
        self.consistent(cell)
    }

    pub fn consistent(&self, cell: Cell) -> bool {
        let reference = self.get(cell);

        if Token::None == reference {
            return true;
        }

        let mut found = false;
        for token in self.0.row(cell.row) {
            if token == reference as u8 {
                if found {
                    return false;
                } else {
                    found = true;
                }
            }
        }

        found = false;
        for token in self.0.column(cell.col) {
            if token == reference as u8 {
                if found {
                    return false;
                } else {
                    found = true;
                }
            }
        }

        found = false;
        for token in self.0.cluster(cell.row / 3 + cell.col / 3) {
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

    pub fn list_inconsistencies(&self) -> Vec<Cell> {
        let mut inconsistencies = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                let cell = Cell { row, col };
                if !self.consistent(cell) {
                    inconsistencies.push(cell);
                }
            }
        }
        inconsistencies
    }

    fn reverse(&mut self) {
        let temp = self.0;
        for i in 0..self.0.len() {
            self.0[i] = temp[temp.len() - i - 1];
        }
    }

    fn rotate(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, token) in other.column(i).enumerate() {
                self.0[usize::from(i) * 9 + index] = token;
            }
        }
    }

    fn mirror_columns(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, token) in other.column(i).enumerate() {
                self.0[index * 9 + (8 - usize::from(i))] = token;
            }
        }
    }

    fn mirror_rows(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, token) in other.row(i).enumerate() {
                self.0[(8 - usize::from(i)) * 9 + index] = token;
            }
        }
    }

    fn swap_columns(&mut self, cluster_column: u8, pivot: u8) {
        if cluster_column > 2 {
            panic!("There are only three cluster columns: {}", cluster_column);
        }

        if pivot > 2 {
            panic!("There are only three columns per cluster: {}", pivot);
        }

        let other = self.0;
        let col1 = usize::from(((pivot + 1) % 3) + cluster_column * 3);
        let col2 = usize::from(((pivot + 2) % 3) + cluster_column * 3);

        for row in 0..9 {
            let row_ref = row * 9;
            self.0[row_ref + col1] = other[row_ref + col2];
            self.0[row_ref + col2] = other[row_ref + col1];
        }
    }

    fn swap_rows(&mut self, cluster_row: u8, pivot: u8) {
        if cluster_row > 2 {
            panic!("There are only three cluster rows: {}", cluster_row);
        }

        if pivot > 2 {
            panic!("There are only three rows per cluster: {}", pivot);
        }

        let other = self.0;
        let row1 = usize::from(((pivot + 1) % 3) + cluster_row * 3) * 9;
        let row2 = usize::from(((pivot + 2) % 3) + cluster_row * 3) * 9;

        self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    }

    fn swap_column_cluster(&mut self, pivot: u8) {
        if pivot > 2 {
            panic!("There are only three cluster columns: {}", pivot);
        }

        let other = self.0;
        let col1 = usize::from(((pivot + 1) % 3) * 3);
        let col2 = usize::from(((pivot + 1) % 3) * 3);

        for row in 0..9 {
            let row_ref = row * 9;
            self.0[row_ref + col1] = other[row_ref + col2];
            self.0[row_ref + col1 + 1] = other[row_ref + col2 + 1];
            self.0[row_ref + col1 + 2] = other[row_ref + col2 + 2];

            self.0[row_ref + col2] = other[row_ref + col1];
            self.0[row_ref + col2 + 1] = other[row_ref + col1 + 1];
            self.0[row_ref + col2 + 2] = other[row_ref + col1 + 2];
        }
    }

    fn swap_row_cluster(&mut self, pivot: u8) {
        if pivot > 2 {
            panic!("There are only three cluster rows: {}", pivot);
        }

        let other = self.0;
        let row1 = usize::from(((pivot + 1) % 3) * 3) * 9;
        let row2 = usize::from(((pivot + 1) % 3) * 3) * 9;

        self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
        self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
        self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    }

    fn shift(&mut self, amount: u8) {
        if amount > 7 {
            panic!("A cell can only be shifted up to seven places: {}", amount);
        }

        for token in self.0.iter_mut() {
            *token = ((*token + amount) % 9) + 1;
        }
    }
}

impl std::fmt::Display for Board {
    // Allowed because it is more readable
    #[allow(clippy::non_ascii_literal)]
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "┏━━━━━┯━━━━━┯━━━━━┓")?;
        for col in 0..9 {
            write!(fmt, "┃")?;
            for row in 0..8 {
                if row % 3 == 2 {
                    write!(fmt, "{}│", self.get(Cell { row, col }))?;
                } else {
                    write!(fmt, "{} ", self.get(Cell { row, col }))?;
                }
            }
            writeln!(fmt, "{}┃", self.get(Cell { row: 8, col }))?;
            if col < 8 && col % 3 == 2 {
                writeln!(fmt, "┠─────┼─────┼─────┨")?;
            }
        }
        writeln!(fmt, "┗━━━━━┷━━━━━┷━━━━━┛")
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..80 {
            write!(fmt, "{},", self.0[i])?;
        }
        write!(fmt, "{}", self.0[80])
    }
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
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        TokenIterator(1)
    }
}

impl std::convert::From<u8> for Token {
    fn from(token: u8) -> Self {
        match token {
            0 => Token::None,
            1 => Token::One,
            2 => Token::Two,
            3 => Token::Three,
            4 => Token::Four,
            5 => Token::Five,
            6 => Token::Six,
            7 => Token::Seven,
            8 => Token::Eight,
            9 => Token::Nine,
            _ => panic!("Token out of bounds: {}", token),
        }
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

struct TokenIterator(u8);

impl std::iter::Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 9 {
            None
        } else {
            let token = Some(Token::from(self.0));
            self.0 += 1;
            token
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cell {
    row: u8,
    col: u8,
}

impl Cell {
    #[inline]
    pub fn new(row: u8, col: u8) -> Self {
        if row > 8 || col > 8 {
            panic!("Cell aout of bounds (row: {}, col: {})", row, col);
        }
        Self { row, col }
    }

    fn as_linear(self) -> usize {
        usize::from(self.row * 9 + self.col)
    }
}

impl std::convert::From<u8> for Cell {
    fn from(index: u8) -> Self {
        if index > 80 {
            panic!("Index out of bounds: {}", index);
        }
        Self::new(index / 9, index % 9)
    }
}

impl std::convert::From<&u8> for Cell {
    fn from(index: &u8) -> Self {
        Self::from(*index)
    }
}

// impl std::cmp::Ord for Cell {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.as_linear().cmp(&other.as_linear())
//     }
// }

struct RowIterator<'a> {
    board: &'a [u8; 81],
    base: usize,
    index: usize,
}

impl std::iter::Iterator for RowIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 9 {
            let token = Some(self.board[self.base * 9 + self.index]);
            self.index += 1;
            token
        } else {
            None
        }
    }
}

trait IntoRowIterator {
    fn row(&self, index: u8) -> RowIterator<'_>;
}

impl IntoRowIterator for [u8; 81] {
    fn row(&self, index: u8) -> RowIterator<'_> {
        if index > 8 {
            panic!("Invalid row: {}", index);
        }
        RowIterator {
            board: &self,
            base: usize::from(index),
            index: 0,
        }
    }
}

struct ColumnIterator<'a> {
    board: &'a [u8; 81],
    base: usize,
    index: usize,
}

impl std::iter::Iterator for ColumnIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 9 {
            let token = Some(self.board[self.base + self.index * 9]);
            self.index += 1;
            token
        } else {
            None
        }
    }
}

trait IntoColumnIterator {
    fn column(&self, index: u8) -> ColumnIterator<'_>;
}

impl IntoColumnIterator for [u8; 81] {
    fn column(&self, index: u8) -> ColumnIterator<'_> {
        if index > 8 {
            panic!("Invalid column: {}", index);
        }
        ColumnIterator {
            board: &self,
            base: usize::from(index),
            index: 0,
        }
    }
}

struct ClusterIterator<'a> {
    board: &'a [u8; 81],
    base: usize,
    index: usize,
}

impl std::iter::Iterator for ClusterIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 9 {
            let token = Some(self.board[self.base + ((self.index / 3) * 9) + (self.index % 3)]);
            self.index += 1;
            token
        } else {
            None
        }
    }
}

trait IntoClusterIterator {
    fn cluster(&self, index: u8) -> ClusterIterator<'_>;
}

impl IntoClusterIterator for [u8; 81] {
    fn cluster(&self, index: u8) -> ClusterIterator<'_> {
        if index > 8 {
            panic!("Invalid cluster: {}", index);
        }
        let col = 3 * (index % 3);
        let row = 3 * (index / 3);
        ClusterIterator {
            board: &self,
            base: usize::from(row * 9 + col),
            index: 0,
        }
    }
}

// Allowed because this is a test
#[allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod test {
    use super::{Board, Cell, IntoClusterIterator, IntoColumnIterator, IntoRowIterator, Token};

    fn sequential_board() -> Board {
        let mut board = [0; 81];
        for i in 0..81 {
            board[usize::from(i)] = i;
        }
        Board(board)
    }

    #[test]
    fn get() {
        let board = Board::initialize_base();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(
                    board.get(Cell { row, col }) as u8,
                    board.0[usize::from(row * 9 + col)]
                );
            }
        }
    }

    #[test]
    fn set() {
        let mut board = Board::initialize_base();
        assert!(board.set(Cell { row: 4, col: 5 }, Token::One));

        for row in 0..9 {
            for col in 0..9 {
                if row == 4 && col == 5 {
                    assert_eq!(board.get(Cell { row, col }), Token::One);
                } else {
                    assert_eq!(
                        board.get(Cell { row, col }) as u8,
                        board.0[usize::from(row * 9 + col)]
                    );
                }
            }
        }

        assert!(!board.set(Cell::new(2, 0), Token::One));
    }

    #[test]
    fn reverse() {
        let mut board = sequential_board();
        board.reverse();

        for i in 0..81 {
            assert_eq!(board.0[usize::from(i)], 80 - i);
        }
    }

    #[test]
    fn rotate() {
        let mut board = sequential_board();
        board.rotate();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.0[usize::from(row * 9 + col)], col * 9 + row);
            }
        }
    }

    #[test]
    fn mirror_columns() {
        let mut board = sequential_board();
        board.mirror_columns();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.0[usize::from(row * 9 + col)], row * 9 + (8 - col));
            }
        }
    }

    #[test]
    fn mirror_rows() {
        let mut board = sequential_board();
        board.mirror_rows();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.0[usize::from(row * 9 + col)], (8 - row) * 9 + col);
            }
        }
    }

    #[test]
    fn swap_columns() {
        let mut board = Board::initialize_base();
        board.swap_columns(0, 2);
        let expected = Board::initialize_base();

        for row in 0..9 {
            for col in 0..9 {
                if col == 0 {
                    assert_eq!(
                        board.get(Cell { row, col }),
                        expected.get(Cell { row, col: col + 1 })
                    );
                } else if col == 1 {
                    assert_eq!(
                        board.get(Cell { row, col }),
                        expected.get(Cell { row, col: col - 1 })
                    );
                } else {
                    assert_eq!(
                        board.get(Cell { row, col }),
                        expected.get(Cell { row, col })
                    );
                }
            }
        }
    }

    #[test]
    fn swap_rows() {
        let mut board = Board::initialize_base();
        board.swap_rows(0, 2);
        let expected = Board::initialize_base();

        for row in 0..9 {
            for col in 0..9 {
                if row == 0 {
                    assert_eq!(
                        board.get(Cell { row, col }),
                        expected.get(Cell { row: row + 1, col })
                    );
                } else if row == 1 {
                    assert_eq!(
                        board.get(Cell { row, col }),
                        expected.get(Cell { row: row - 1, col })
                    );
                } else {
                    assert_eq!(
                        board.get(Cell { row, col }),
                        expected.get(Cell { row, col })
                    );
                }
            }
        }
    }

    #[test]
    fn swap_column_cluster() {
        let mut board = Board::initialize_base();
        board.swap_column_cluster(2);
        board.swap_column_cluster(1);
        let expected = Board::initialize_base();

        for row in 0..9 {
            for col_index in 0..9 {
                let col = (col_index + 1) % 3;
                assert_eq!(
                    board.get(Cell { row, col }),
                    expected.get(Cell { row, col })
                );
            }
        }
    }

    #[test]
    fn swap_row_cluster() {
        let mut board = Board::initialize_base();
        board.swap_row_cluster(2);
        board.swap_row_cluster(1);
        let expected = Board::initialize_base();

        for row_index in 0..9 {
            for col in 0..9 {
                let row = (row_index + 1) % 3;
                assert_eq!(
                    board.get(Cell { row, col }),
                    expected.get(Cell { row, col })
                );
            }
        }
    }

    #[test]
    fn shift() {
        let mut board = sequential_board();
        board.shift(1);

        for i in 0..81 {
            assert_eq!(board.0[usize::from(i)], (i + 1) % 9 + 1);
        }
    }

    #[test]
    fn row_iterator() {
        let board = sequential_board();

        for i in 0..9 {
            let iter = board.0.row(i);
            for (index, token) in iter.enumerate() {
                assert_eq!(token, i * 9 + index as u8);
            }
        }
    }

    #[test]
    fn column_iterator() {
        let board = sequential_board();

        for i in 0..9 {
            let iter = board.0.column(i);
            for (index, token) in iter.enumerate() {
                assert_eq!(token, index as u8 * 9 + i);
            }
        }
    }

    #[test]
    fn cluster_iterator() {
        let board = sequential_board();

        for i in 0..9 {
            let base = (27 * (i / 3)) + 3 * (i % 3);
            let iter = board.0.cluster(i);
            for (index, token) in iter.enumerate() {
                let byte_index = index as u8;
                let expected = base + (9 * (byte_index / 3)) + (byte_index % 3);
                assert_eq!(token, expected);
            }
        }
    }

    #[test]
    fn token_iterator() {
        for (index, token) in Token::iter().enumerate() {
            println!("Left: {}, Right: {}", index, token);
            assert_eq!((index + 1) as u8, token as u8);
        }
    }

    #[test]
    fn cell_as_linear() {
        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(Cell::new(row, col).as_linear(), usize::from(row * 9 + col));
            }
        }
    }

    #[test]
    fn cell_ordering() {
        use rand::seq::SliceRandom;

        let mut rng = rand::thread_rng();
        let mut cells = [Cell::from(0); 81];
        for i in 0..81 {
            cells[usize::from(i)] = Cell::from(i);
        }

        cells.shuffle(&mut rng);

        let mut shuffled = false;
        for (index, cell) in cells.iter().enumerate() {
            if cell.as_linear() != index {
                shuffled = true;
                break;
            }
        }
        assert!(shuffled);
        cells.sort();

        for (index, cell) in cells.iter().enumerate() {
            assert_eq!(cell.as_linear(), index);
        }
    }

    #[test]
    fn consistent() {
        use rand::Rng;

        let mut board = Board::initialize_base();
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.reverse();
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.rotate();
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.mirror_columns();
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.mirror_rows();
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.swap_columns(2, 1);
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.swap_rows(1, 2);
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.swap_column_cluster(1);
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.swap_row_cluster(1);
        assert_eq!(board.list_inconsistencies().len(), 0);
        board.shift(1);
        assert_eq!(board.list_inconsistencies().len(), 0);

        let lucky_index = rand::thread_rng().gen::<u8>() % 81;
        let lucky_cell = Cell::from(lucky_index);
        let cell = board.get(lucky_cell) as u8;
        board.0[usize::from(lucky_index)] = ((cell + 1) % 9) + 1;
        assert!(!board.consistent(lucky_cell));
        assert!(!board.list_inconsistencies().is_empty());
    }
}
