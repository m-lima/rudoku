use super:{Board, Token};
use crate::index::{ColumnIndexer, RowIndexer};

fn shift(board: &mut Board, amount: u8) {
    for token in board.iter_mut() {
        if token != &Token::None {
            *token = Token::from(((*token as u8 + amount - 1) % 9) + 1);
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

#[cfg(test)]
mod tests {
    #[test]
    fn shift() {
        #[rustfmt::skip]
        let jig: [usize; 81] = [
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ];

        #[rustfmt::skip]
        let expected: [usize; 81] = [
            0,1,2,3,4,5,6,7,8,
            9,0,1,2,3,4,5,6,7,
            8,9,0,1,2,3,4,5,6,
            7,8,9,0,1,2,3,4,5,
            6,7,8,9,0,1,2,3,4,
            5,6,7,8,9,0,1,2,3,
            4,5,6,7,8,9,0,1,2,
            3,4,5,6,7,8,9,0,1,
            2,3,4,5,6,7,8,9,0,
            ];

    }

    #[test]
    fn rotate() {
    }

    #[test]
    fn mirror_columns() {
    }

    #[test]
    fn mirror_rows() {
    }

    #[test]
    fn swap_columns() {
    }

    #[test]
    fn swap_rows() {
    }

    #[test]
    fn swap_column_cluster() {
    }

    #[test]
    fn swap_row_cluster() {
    }
}
