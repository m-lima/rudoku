#[derive(Copy, Clone)]
pub struct Builder([u8; 81]);

impl Builder {
    pub fn build(self) -> super::Board {
        Board
    }

    pub fn new() -> Self {
        Self([0; 81])
    }

    #[cfg(test)]
    pub fn sequential() -> Self {
        let mut board = [0; 81];
        for i in 0..81 {
            board[usize::from(i)] = i;
        }
        Self(board)
    }

    #[cfg(test)]
    pub fn consistent() -> Self {
        Self([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7, 8, 9, 1, 2, 3, 7, 8, 9, 1, 2, 3, 4, 5, 6, 2, 3,
            4, 5, 6, 7, 8, 9, 1, 5, 6, 7, 8, 9, 1, 2, 3, 4, 8, 9, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6,
            7, 8, 9, 1, 2, 6, 7, 8, 9, 1, 2, 3, 4, 5, 9, 1, 2, 3, 4, 5, 6, 7, 8,
        ])
    }

    pub fn shuffle(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..128 {
            match rng.gen::<u8>() % 9 {
                1 => self.rotate(),
                2 => self.mirror_columns(),
                3 => self.mirror_rows(),
                4 => self.swap_columns(rng.gen::<u8>() % 3, rng.gen::<u8>() % 3),
                5 => self.swap_rows(rng.gen::<u8>() % 3, rng.gen::<u8>() % 3),
                6 => self.swap_column_sector(rng.gen::<u8>() % 3),
                7 => self.swap_row_sector(rng.gen::<u8>() % 3),
                8 => self.shift(rng.gen::<u8>() % 7),
                _ => unreachable!(),
            }
        }
    }

    pub fn rotate(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, token) in other.column(i).enumerate() {
                self.0[usize::from(i) * 9 + index] = token;
            }
        }
    }

    pub fn mirror_columns(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, token) in other.column(i).enumerate() {
                self.0[index * 9 + (8 - usize::from(i))] = token;
            }
        }
    }

    pub fn mirror_rows(&mut self) {
        let other = self.0;
        for i in 0..9 {
            for (index, token) in other.row(i).enumerate() {
                self.0[(8 - usize::from(i)) * 9 + index] = token;
            }
        }
    }

    pub fn swap_columns(&mut self, sector_column: u8, pivot: u8) {
        assert!(
            sector_column < 3,
            "There are only three sector columns: {}",
            sector_column
        );
        assert!(
            pivot < 3,
            "There are only three columns per sector: {}",
            pivot
        );

        let other = self.0;
        let col1 = usize::from(((pivot + 1) % 3) + sector_column * 3);
        let col2 = usize::from(((pivot + 2) % 3) + sector_column * 3);

        for row in 0..9 {
            let row_ref = row * 9;
            self.0[row_ref + col1] = other[row_ref + col2];
            self.0[row_ref + col2] = other[row_ref + col1];
        }
    }

    pub fn swap_rows(&mut self, sector_row: u8, pivot: u8) {
        assert!(
            sector_row < 3,
            "There are only three sector rows: {}",
            sector_row
        );
        assert!(pivot < 3, "There are only three rows per sector: {}", pivot);

        let other = self.0;
        let row1 = usize::from(((pivot + 1) % 3) + sector_row * 3) * 9;
        let row2 = usize::from(((pivot + 2) % 3) + sector_row * 3) * 9;

        self.0[row1..(9 + row1)].clone_from_slice(&other[row2..(9 + row2)]);
        self.0[row2..(9 + row2)].clone_from_slice(&other[row1..(9 + row1)]);
    }

    pub fn swap_column_sector(&mut self, pivot: u8) {
        assert!(pivot < 3, "There are only three sector columns: {}", pivot);

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

    pub fn swap_row_sector(&mut self, pivot: u8) {
        assert!(pivot < 3, "There are only three sector rows: {}", pivot);

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
        assert!(
            amount < 8,
            "A cell can only be shifted up to seven places: {}",
            amount
        );

        for token in self.0.iter_mut() {
            *token = ((*token + amount) % 9) + 1;
        }
    }
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

struct SectorIterator<'a> {
    board: &'a [u8; 81],
    base: usize,
    index: usize,
}

impl std::iter::Iterator for SectorIterator<'_> {
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

trait IntoSectorIterator {
    fn sector(&self, index: u8) -> SectorIterator<'_>;
}

impl IntoSectorIterator for [u8; 81] {
    fn sector(&self, index: u8) -> SectorIterator<'_> {
        if index > 8 {
            panic!("Invalid sector: {}", index);
        }
        let col = 3 * (index % 3);
        let row = 3 * (index / 3);
        SectorIterator {
            board: &self,
            base: usize::from(row * 9 + col),
            index: 0,
        }
    }
}

// Allowed because this is a test
#[allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use super::{Builder, Cell, IntoColumnIterator, IntoRowIterator, IntoSectorIterator};

    #[test]
    fn rotate() {
        let mut board = Builder::sequential();
        board.rotate();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.0[usize::from(row * 9 + col)], col * 9 + row);
            }
        }
    }

    #[test]
    fn mirror_columns() {
        let mut board = Builder::sequential();
        board.mirror_columns();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.0[usize::from(row * 9 + col)], row * 9 + (8 - col));
            }
        }
    }

    #[test]
    fn mirror_rows() {
        let mut board = Builder::sequential();
        board.mirror_rows();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.0[usize::from(row * 9 + col)], (8 - row) * 9 + col);
            }
        }
    }

    #[test]
    fn swap_columns() {
        let mut board = Builder::consistent();
        board.swap_columns(0, 2);
        let expected = Builder::consistent();

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
        let mut board = Builder::consistent();
        board.swap_rows(0, 2);
        let expected = Builder::consistent();

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
    fn swap_column_sector() {
        let mut board = Builder::consistent();
        board.swap_column_sector(2);
        board.swap_column_sector(1);
        let expected = Builder::consistent();

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
    fn swap_row_sector() {
        let mut board = Builder::consistent();
        board.swap_row_sector(2);
        board.swap_row_sector(1);
        let expected = Builder::consistent();

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
        let mut board = Builder::sequential();
        board.shift(1);

        for i in 0..81 {
            assert_eq!(board.0[usize::from(i)], (i + 1) % 9 + 1);
        }
    }

    #[test]
    fn row_iterator() {
        let board = Builder::sequential();

        for i in 0..9 {
            let iter = board.0.row(i);
            for (index, token) in iter.enumerate() {
                assert_eq!(token, i * 9 + index as u8);
            }
        }
    }

    #[test]
    fn column_iterator() {
        let board = Builder::sequential();

        for i in 0..9 {
            let iter = board.0.column(i);
            for (index, token) in iter.enumerate() {
                assert_eq!(token, index as u8 * 9 + i);
            }
        }
    }

    #[test]
    fn sector_iterator() {
        let board = Builder::sequential();

        for i in 0..9 {
            let base = (27 * (i / 3)) + 3 * (i % 3);
            let iter = board.0.sector(i);
            for (index, token) in iter.enumerate() {
                let byte_index = index as u8;
                let expected = base + (9 * (byte_index / 3)) + (byte_index % 3);
                assert_eq!(token, expected);
            }
        }
    }

    //TODO Use ops:: instead of game::
    #[test]
    fn transforms_are_consistent() {
        use rand::Rng;

        let mut game = super::super::Game {
            board: Builder::consistent(),
        };

        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.reverse();
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.rotate();
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.mirror_columns();
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.mirror_rows();
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.swap_columns(2, 1);
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.swap_rows(1, 2);
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.swap_column_sector(1);
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.swap_row_sector(1);
        assert_eq!(game.list_inconsistencies().len(), 0);
        game.board.shift(1);
        assert_eq!(game.list_inconsistencies().len(), 0);

        let lucky_index = rand::thread_rng().gen::<u8>() % 81;
        let lucky_cell = Cell::from(lucky_index);
        let cell = game.board.get(lucky_cell) as u8;
        game.board.0[usize::from(lucky_index)] = ((cell + 1) % 9) + 1;
        assert!(!game.list_inconsistencies().is_empty());
    }
}
