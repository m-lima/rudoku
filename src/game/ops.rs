use super::{Board, Cell, Token};
use crate::index::{ColumnIndexer, RowIndexer, SectorIndexer};

pub fn consistent(board: &Board, cell: Cell) -> bool {
    let reference = board[cell.index()];

    if reference == Token::None {
        return true;
    }

    for current in RowIndexer::new(cell.row()) {
        if current == cell {
            continue;
        }

        if board[current.index()] == reference {
            return false;
        }
    }

    for current in ColumnIndexer::new(cell.column()) {
        if current == cell {
            continue;
        }

        if board[current.index()] == reference {
            return false;
        }
    }

    for current in SectorIndexer::new(cell.sector()) {
        if current == cell {
            continue;
        }

        if board[current.index()] == reference {
            return false;
        }
    }

    true
}

#[cfg(test)]
pub fn consistent_board() -> Board {
    #[rustfmt::skip]
    let consistent = super::tokenize([
        1,2,3,4,5,6,7,8,9,
        4,5,6,7,8,9,1,2,3,
        7,8,9,1,2,3,4,5,6,
        2,3,4,5,6,7,8,9,1,
        5,6,7,8,9,1,2,3,4,
        8,9,1,2,3,4,5,6,7,
        3,4,5,6,7,8,9,1,2,
        6,7,8,9,1,2,3,4,5,
        9,1,2,3,4,5,6,7,8,
    ]);
    consistent
}

#[cfg(test)]
mod tests {
    use super::super::tokenize;
    use super::Cell;
    use crate::index::BoardIndexer;

    #[test]
    fn full_consistency() {
        let jig = super::consistent_board();
        for cell in BoardIndexer::new() {
            assert!(super::consistent(&jig, cell));
        }
    }

    #[test]
    fn row_inconsistency() {
        #[rustfmt::skip]
        let jig = tokenize([
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            2,3,4,5,1,7,8,9,1,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
        ]);

        assert!(!super::consistent(&jig, Cell::new(3, 4)));
    }

    #[test]
    fn column_inconsistency() {
        #[rustfmt::skip]
        let jig = tokenize([
            0,0,0,4,0,0,0,0,0,
            0,0,0,7,0,0,0,0,0,
            0,0,0,1,0,0,0,0,0,
            0,0,0,9,0,0,0,0,0,
            0,0,0,8,0,0,0,0,0,
            0,0,0,2,0,0,0,0,0,
            0,0,0,6,0,0,0,0,0,
            0,0,0,9,0,0,0,0,0,
            0,0,0,3,0,0,0,0,0,
        ]);

        assert!(!super::consistent(&jig, Cell::new(3, 3)));
    }

    #[test]
    fn sector_inconsistency() {
        #[rustfmt::skip]
        let jig = tokenize([
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,5,6,7,0,0,0,
            0,0,0,8,2,1,0,0,0,
            0,0,0,2,3,4,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
        ]);

        assert!(!super::consistent(&jig, Cell::new(4, 4)));
    }
}
