use crate::game::Cell;

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
