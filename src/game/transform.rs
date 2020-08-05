use super::{Board, Token};
use crate::index::{ColumnIndexer, RowIndexer};

pub fn shift(board: &mut Board, amount: u8) {
    for token in board.iter_mut() {
        if token != &Token::None {
            *token = Token::from(((*token as u8 + amount - 1) % 9) + 1);
        }
    }
}

pub fn rotate(board: &mut Board) {
    let other = *board;
    for i in 0..9 {
        let col = ColumnIndexer::new(i);
        let row = RowIndexer::new(i);
        for (col, row) in col.zip(row) {
            board[col.index()] = other[row.index()];
        }
    }
}

pub fn mirror_columns(board: &mut Board) {
    let other = *board;
    for i in 0..9 {
        let first = ColumnIndexer::new(i);
        let second = ColumnIndexer::new(8 - i);
        for (first, second) in first.zip(second) {
            board[first.index()] = other[second.index()];
        }
    }
}

pub fn mirror_rows(board: &mut Board) {
    let other = *board;
    for i in 0..9 {
        let first = RowIndexer::new(i);
        let second = RowIndexer::new(8 - i);
        for (first, second) in first.zip(second) {
            board[first.index()] = other[second.index()];
        }
    }
}

pub fn swap_columns(board: &mut Board, sector_column: usize, pivot: usize) {
    let other = *board;
    let col1 = ((pivot + 1) % 3) + sector_column * 3;
    let col2 = ((pivot + 2) % 3) + sector_column * 3;

    let first = ColumnIndexer::new(col1);
    let second = ColumnIndexer::new(col2);
    for (first, second) in first.zip(second) {
        board[first.index()] = other[second.index()];
        board[second.index()] = other[first.index()];
    }
}

pub fn swap_rows(board: &mut Board, sector_row: usize, pivot: usize) {
    let other = *board;
    let row1 = (((pivot + 1) % 3) + sector_row * 3) * 9;
    let row2 = (((pivot + 2) % 3) + sector_row * 3) * 9;

    board[row1..(9 + row1)].copy_from_slice(&other[row2..(9 + row2)]);
    board[row2..(9 + row2)].copy_from_slice(&other[row1..(9 + row1)]);
}

pub fn swap_column_sector(board: &mut Board, pivot: usize) {
    let other = *board;
    let col1 = ((pivot + 1) % 3) * 3;
    let col2 = ((pivot + 2) % 3) * 3;

    let first = ColumnIndexer::new(col1);
    let second = ColumnIndexer::new(col2);
    for (first, second) in first.zip(second) {
        let first = first.index();
        let second = second.index();
        board[first..(3 + first)].copy_from_slice(&other[second..(3 + second)]);
        board[second..(3 + second)].copy_from_slice(&other[first..(3 + first)]);
    }
}

pub fn swap_row_sector(board: &mut Board, pivot: usize) {
    let other = *board;
    let row1 = (((pivot + 1) % 3) * 3) * 9;
    let row2 = (((pivot + 2) % 3) * 3) * 9;

    board[row1..(27 + row1)].copy_from_slice(&other[row2..(27 + row2)]);
    board[row2..(27 + row2)].copy_from_slice(&other[row1..(27 + row1)]);
}

#[cfg(test)]
mod tests {
    use super::super::tokenize;

    #[test]
    fn shift() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            0,3,4,5,6,7,8,9,1,
            2,0,3,4,5,6,7,8,9,
            1,2,0,3,4,5,6,7,8,
            9,1,2,0,3,4,5,6,7,
            8,9,1,2,0,3,4,5,6,
            7,8,9,1,2,0,3,4,5,
            6,7,8,9,1,2,0,3,4,
            5,6,7,8,9,1,2,0,3,
            4,5,6,7,8,9,1,2,0,
            ]);

        super::shift(&mut jig, 2);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn rotate() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,1,1,1,
            2,2,2,2,2,2,2,2,2,
            3,3,3,3,3,3,3,3,3,
            4,4,4,4,4,4,4,4,4,
            5,5,5,5,5,5,5,5,5,
            6,6,6,6,6,6,6,6,6,
            7,7,7,7,7,7,7,7,7,
            8,8,8,8,8,8,8,8,8,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            ]);

        super::rotate(&mut jig);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn mirror_columns() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            8,7,6,5,4,3,2,1,0,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            0,1,2,3,4,5,6,7,8,
            ]);

        super::mirror_columns(&mut jig);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn mirror_rows() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,1,1,1,
            2,2,2,2,2,2,2,2,2,
            3,3,3,3,3,3,3,3,3,
            4,4,4,4,4,4,4,4,4,
            5,5,5,5,5,5,5,5,5,
            6,6,6,6,6,6,6,6,6,
            7,7,7,7,7,7,7,7,7,
            8,8,8,8,8,8,8,8,8,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            8,8,8,8,8,8,8,8,8,
            7,7,7,7,7,7,7,7,7,
            6,6,6,6,6,6,6,6,6,
            5,5,5,5,5,5,5,5,5,
            4,4,4,4,4,4,4,4,4,
            3,3,3,3,3,3,3,3,3,
            2,2,2,2,2,2,2,2,2,
            1,1,1,1,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,
            ]);

        super::mirror_rows(&mut jig);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn swap_columns() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            0,1,2,5,4,3,6,7,8,
            9,0,1,4,3,2,5,6,7,
            8,9,0,3,2,1,4,5,6,
            7,8,9,2,1,0,3,4,5,
            6,7,8,1,0,9,2,3,4,
            5,6,7,0,9,8,1,2,3,
            4,5,6,9,8,7,0,1,2,
            3,4,5,8,7,6,9,0,1,
            2,3,4,7,6,5,8,9,0,
            ]);

        super::swap_columns(&mut jig, 1, 1);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn swap_rows() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            0,1,2,3,4,5,6,7,8,
            8,9,0,1,2,3,4,5,6,
            9,0,1,2,3,4,5,6,7,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ]);

        super::swap_rows(&mut jig, 0, 0);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn swap_column_sector() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            6,7,8,3,4,5,0,1,2,
            5,6,7,2,3,4,9,0,1,
            4,5,6,1,2,3,8,9,0,
            3,4,5,0,1,2,7,8,9,
            2,3,4,9,0,1,6,7,8,
            1,2,3,8,9,0,5,6,7,
            0,1,2,7,8,9,4,5,6,
            9,0,1,6,7,8,3,4,5,
            8,9,0,5,6,7,2,3,4,
            ]);

        super::swap_column_sector(&mut jig, 1);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }

    #[test]
    fn swap_row_sector() {
        #[rustfmt::skip]
        let mut jig  = tokenize([
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ]);

        #[rustfmt::skip]
        let expected = tokenize([
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            ]);

        super::swap_row_sector(&mut jig, 0);
        for i in 0..81 {
            assert_eq!(jig[i], expected[i]);
        }
    }
}
