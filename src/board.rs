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

    pub fn get(&self, cell: Cell) -> Option<u8> {
        let value = self.0[cell.as_linear()];
        if value == 0 {
            None
        } else {
            Some(value)
        }
    }

    fn get_raw(&self, cell: Cell) -> u8 {
        self.0[cell.as_linear()]
    }

    pub fn set(&mut self, cell: Cell, value: u8) -> bool {
        if value < 1 || value > 9 {
            panic!("Invalid value: {}", value);
        }
        self.0[cell.as_linear()] = value;
        self.consistent(cell)
    }

    pub fn clear(&mut self, cell: Cell) {
        self.0[cell.as_linear()] = 0;
    }

    pub fn consistent(&self, cell: Cell) -> bool {
        let reference = self.get_raw(cell);
        if reference > 0 {
            let mut found = false;
            for value in self.0.row(cell.row) {
                if value == reference {
                    if found {
                        return false;
                    } else {
                        found = true;
                    }
                }
            }

            found = false;
            for value in self.0.column(cell.col) {
                if value == reference {
                    if found {
                        return false;
                    } else {
                        found = true;
                    }
                }
            }

            found = false;
            for value in self.0.cluster(cell.row / 3 + cell.col / 3) {
                if value == reference {
                    if found {
                        return false;
                    } else {
                        found = true;
                    }
                }
            }

            true
        } else {
            true
        }
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
            for (index, value) in other.column(i).enumerate() {
                self.0[usize::from(i) * 9 + index] = value;
            }
        }
    }

    fn mirror_columns(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, value) in other.column(i).enumerate() {
                self.0[index * 9 + (8 - usize::from(i))] = value;
            }
        }
    }

    fn mirror_rows(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, value) in other.row(i).enumerate() {
                self.0[(8 - usize::from(i)) * 9 + index] = value;
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

        for value in self.0.iter_mut() {
            *value = ((*value + amount) % 9) + 1;
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
                    write!(
                        fmt,
                        "{}│",
                        self.get(Cell { row, col })
                            .map_or(' ', |c| (c + 48) as char)
                    )?;
                } else {
                    write!(
                        fmt,
                        "{} ",
                        self.get(Cell { row, col })
                            .map_or(' ', |c| (c + 48) as char)
                    )?;
                }
            }
            writeln!(
                fmt,
                "{}┃",
                self.get(Cell { row: 8, col })
                    .map_or(' ', |c| (c + 48) as char)
            )?;
            if col < 8 && col % 3 == 2 {
                writeln!(fmt, "┠─────┼─────┼─────┨")?;
            }
        }
        writeln!(fmt, "┗━━━━━┷━━━━━┷━━━━━┛")
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..81 {
            write!(fmt, "{},", self.0[i])?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    row: u8,
    col: u8,
}

impl Cell {
    pub fn from_index(index: u8) -> Self {
        Self::new(index / 9, index % 9)
    }

    pub fn new(row: u8, col: u8) -> Self {
        if row > 8 || col > 8 {
            panic!("Cannot linearize (row: {}, col: {}", row, col);
        }
        Self { row, col }
    }

    fn as_linear(self) -> usize {
        if self.row > 8 || self.col > 8 {
            panic!("Cannot linearize (row: {}, col: {}", self.row, self.col);
        }
        usize::from(self.row * 9 + self.col)
    }

    // #[inline]
    // pub const fn row(self) -> u8 {
    //     self.row
    // }

    // #[inline]
    // pub const fn col(self) -> u8 {
    //     self.col
    // }
}

struct RowIterator<'a> {
    board: &'a [u8; 81],
    base: usize,
    index: usize,
}

impl std::iter::Iterator for RowIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 9 {
            let value = Some(self.board[self.base * 9 + self.index]);
            self.index += 1;
            value
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
            let value = Some(self.board[self.base + self.index * 9]);
            self.index += 1;
            value
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
            let value = Some(self.board[self.base + ((self.index / 3) * 9) + (self.index % 3)]);
            self.index += 1;
            value
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
    use super::{Board, Cell, IntoClusterIterator, IntoColumnIterator, IntoRowIterator};

    fn sequential_board() -> Board {
        let mut board = [0; 81];
        for i in 0..81 {
            board[usize::from(i)] = i;
        }
        Board(board)
    }

    #[test]
    fn get() {
        let board = sequential_board();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.get_raw(Cell { row, col }), row * 9 + col);
            }
        }
    }

    #[test]
    fn set() {
        let mut board = sequential_board();
        assert!(board.set(Cell { row: 4, col: 5 }, 1));

        for row in 0..9 {
            for col in 0..9 {
                if row == 4 && col == 5 {
                    assert_eq!(board.get_raw(Cell { row, col }), 1);
                } else {
                    assert_eq!(board.get_raw(Cell { row, col }), row * 9 + col);
                }
            }
        }

        assert!(!board.set(Cell::new(2, 0), 1));
    }

    #[test]
    fn clear() {
        let mut board = sequential_board();
        board.clear(Cell { row: 4, col: 5 });

        for row in 0..9 {
            for col in 0..9 {
                if row == 4 && col == 5 {
                    assert_eq!(board.get_raw(Cell { row, col }), 0);
                } else {
                    assert_eq!(board.get_raw(Cell { row, col }), row * 9 + col);
                }
            }
        }
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
                assert_eq!(board.get_raw(Cell { row, col }), col * 9 + row);
            }
        }
    }

    #[test]
    fn mirror_columns() {
        let mut board = sequential_board();
        board.mirror_columns();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.get_raw(Cell { row, col }), row * 9 + (8 - col));
            }
        }
    }

    #[test]
    fn mirror_rows() {
        let mut board = sequential_board();
        board.mirror_rows();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.get_raw(Cell { row, col }), (8 - row) * 9 + col);
            }
        }
    }

    #[test]
    fn swap_columns() {
        let mut board = sequential_board();
        board.swap_columns(0, 2);
        let expected = sequential_board();

        for row in 0..9 {
            for col in 0..9 {
                if col == 0 {
                    assert_eq!(
                        board.get_raw(Cell { row, col }),
                        expected.get_raw(Cell { row, col: col + 1 })
                    );
                } else if col == 1 {
                    assert_eq!(
                        board.get_raw(Cell { row, col }),
                        expected.get_raw(Cell { row, col: col - 1 })
                    );
                } else {
                    assert_eq!(
                        board.get_raw(Cell { row, col }),
                        expected.get_raw(Cell { row, col })
                    );
                }
            }
        }
    }

    #[test]
    fn swap_rows() {
        let mut board = sequential_board();
        board.swap_rows(0, 2);
        let expected = sequential_board();

        for row in 0..9 {
            for col in 0..9 {
                if row == 0 {
                    assert_eq!(
                        board.get_raw(Cell { row, col }),
                        expected.get_raw(Cell { row: row + 1, col })
                    );
                } else if row == 1 {
                    assert_eq!(
                        board.get_raw(Cell { row, col }),
                        expected.get_raw(Cell { row: row - 1, col })
                    );
                } else {
                    assert_eq!(
                        board.get_raw(Cell { row, col }),
                        expected.get_raw(Cell { row, col })
                    );
                }
            }
        }
    }

    #[test]
    fn swap_column_cluster() {
        let mut board = sequential_board();
        board.swap_column_cluster(2);
        board.swap_column_cluster(1);
        let expected = sequential_board();

        for row in 0..9 {
            for col_index in 0..9 {
                let col = (col_index + 1) % 3;
                assert_eq!(
                    board.get_raw(Cell { row, col }),
                    expected.get_raw(Cell { row, col })
                );
            }
        }
    }

    #[test]
    fn swap_row_cluster() {
        let mut board = sequential_board();
        board.swap_row_cluster(2);
        board.swap_row_cluster(1);
        let expected = sequential_board();

        for row_index in 0..9 {
            for col in 0..9 {
                let row = (row_index + 1) % 3;
                assert_eq!(
                    board.get_raw(Cell { row, col }),
                    expected.get_raw(Cell { row, col })
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
            for (index, value) in iter.enumerate() {
                assert_eq!(value, i * 9 + index as u8);
            }
        }
    }

    #[test]
    fn column_iterator() {
        let board = sequential_board();

        for i in 0..9 {
            let iter = board.0.column(i);
            for (index, value) in iter.enumerate() {
                assert_eq!(value, index as u8 * 9 + i);
            }
        }
    }

    #[test]
    fn cluster_iterator() {
        let board = sequential_board();

        for i in 0..9 {
            let base = (27 * (i / 3)) + 3 * (i % 3);
            let iter = board.0.cluster(i);
            for (index, value) in iter.enumerate() {
                let byte_index = index as u8;
                let expected = base + (9 * (byte_index / 3)) + (byte_index % 3);
                assert_eq!(value, expected);
            }
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
        let lucky_cell = Cell::from_index(lucky_index);
        let cell = board.get_raw(lucky_cell);
        board.0[usize::from(lucky_index)] = ((cell + 1) % 9) + 1;
        assert!(!board.consistent(lucky_cell));
        assert!(!board.list_inconsistencies().is_empty());
    }
}
