#[derive(Copy, Clone)]
pub struct Board([u8; 81]);

impl Board {
    pub fn new() -> Self {
        Self([0; 81])
    }

    fn initialize_base() -> Self {
        Self([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7, 8, 9, 1, 2, 3, 7, 8, 9, 1, 2, 3, 4, 5, 6, 2, 3,
            4, 5, 6, 7, 8, 9, 1, 5, 6, 7, 8, 9, 1, 2, 3, 4, 8, 9, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6,
            7, 8, 9, 1, 2, 6, 7, 8, 9, 1, 2, 3, 4, 5, 9, 1, 2, 3, 4, 5, 6, 7, 8,
        ])
    }

    pub fn initialize() -> Self {
        use rand::Rng;
        let mut board = Self::initialize_base();
        let mut rng = rand::thread_rng();
        for _ in 0..128 {
            match rng.gen::<u8>() % 8 {
                0 => board.reverse(),
                1 => board.rotate(),
                2 => board.mirror_columns(),
                3 => board.mirror_rows(),
                4 => board.swap_columns(rng.gen::<u8>() % 3, rng.gen::<u8>() % 3),
                5 => board.swap_rows(rng.gen::<u8>() % 3, rng.gen::<u8>() % 3),
                6 => board.swap_column_cluster(rng.gen::<u8>() % 3),
                7 => board.swap_row_cluster(rng.gen::<u8>() % 3),
                _ => unreachable!(),
            }
        }
        board
    }

    fn get(&self, coordinate: Coordinate) -> u8 {
        self.0[coordinate.as_linear()]
    }

    fn set(&mut self, coordinate: Coordinate, value: u8) {
        if value < 1 || value > 9 {
            panic!("Invalid value: {}", value);
        }
        self.0[coordinate.as_linear()] = value;
    }

    fn clear(&mut self, coordinate: Coordinate) {
        self.0[coordinate.as_linear()] = 0;
    }

    fn consistent(&self, coordinate: Coordinate) -> bool {
        let reference = self.get(coordinate);
        if reference == 0 {
            return true;
        }

        let mut found = false;
        for value in self.0.row(coordinate.row) {
            if value == reference {
                if found {
                    return false;
                } else {
                    found = true;
                }
            }
        }

        let mut found = false;
        for value in self.0.column(coordinate.col) {
            if value == reference {
                if found {
                    return false;
                } else {
                    found = true;
                }
            }
        }

        let mut found = false;
        for value in self.0.cluster(coordinate.row / 3 + coordinate.col / 3) {
            if value == reference {
                if found {
                    return false;
                } else {
                    found = true;
                }
            }
        }

        true
    }

    fn list_inconsistencies(&self) -> Vec<Coordinate> {
        let mut inconsistencies = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                let coordinate = Coordinate { row, col };
                if !self.consistent(coordinate) {
                    inconsistencies.push(coordinate);
                }
            }
        }
        inconsistencies
    }

    fn reverse(&mut self) {
        let temp = self.0.clone();
        for i in 0..self.0.len() {
            self.0[i] = temp[temp.len() - i - 1];
        }
    }

    fn rotate(&mut self) {
        let other = self.0.clone();
        for i in 0..9 {
            for (index, value) in other.column(i).enumerate() {
                self.0[usize::from(i) * 9 + index] = value;
            }
        }
    }

    fn mirror_columns(&mut self) {
        let other = self.0.clone();
        for i in 0..9 {
            for (index, value) in other.column(i).enumerate() {
                self.0[index * 9 + (8 - usize::from(i))] = value;
            }
        }
    }

    fn mirror_rows(&mut self) {
        let other = self.0.clone();
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

        let other = self.0.clone();
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

        let other = self.0.clone();
        let row1 = usize::from(((pivot + 1) % 3) + cluster_row * 3) * 9;
        let row2 = usize::from(((pivot + 2) % 3) + cluster_row * 3) * 9;

        for col in 0..9 {
            self.0[row1 + col] = other[row2 + col];
            self.0[row2 + col] = other[row1 + col];
        }
    }

    fn swap_column_cluster(&mut self, pivot: u8) {
        if pivot > 2 {
            panic!("There are only three cluster columns: {}", pivot);
        }

        let other = self.0.clone();
        let col1 = usize::from(((pivot + 1) % 3) * 3);
        let col2 = usize::from(((pivot + 1) % 3) * 3);

        for row in 0..9 {
            let row_ref = row * 9;
            self.0[row_ref + col1 + 0] = other[row_ref + col2 + 0];
            self.0[row_ref + col1 + 1] = other[row_ref + col2 + 1];
            self.0[row_ref + col1 + 2] = other[row_ref + col2 + 2];

            self.0[row_ref + col2 + 0] = other[row_ref + col1 + 0];
            self.0[row_ref + col2 + 1] = other[row_ref + col1 + 1];
            self.0[row_ref + col2 + 2] = other[row_ref + col1 + 2];
        }
    }

    fn swap_row_cluster(&mut self, pivot: u8) {
        if pivot > 2 {
            panic!("There are only three cluster rows: {}", pivot);
        }

        let other = self.0.clone();
        let row1 = usize::from(((pivot + 1) % 3) * 3) * 9;
        let row2 = usize::from(((pivot + 1) % 3) * 3) * 9;

        for col in 0..9 {
            self.0[row1 + col] = other[row2 + col];
            self.0[row1 + col] = other[row2 + col];
            self.0[row1 + col] = other[row2 + col];

            self.0[row2 + col] = other[row1 + col];
            self.0[row2 + col] = other[row1 + col];
            self.0[row2 + col] = other[row1 + col];
        }
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
    fn row<'a>(&'a self, index: u8) -> RowIterator<'a>;
}

impl IntoRowIterator for [u8; 81] {
    fn row<'a>(&'a self, index: u8) -> RowIterator<'a> {
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
    fn column<'a>(&'a self, index: u8) -> ColumnIterator<'a>;
}

impl IntoColumnIterator for [u8; 81] {
    fn column<'a>(&'a self, index: u8) -> ColumnIterator<'a> {
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
    fn cluster<'a>(&'a self, index: u8) -> ClusterIterator<'a>;
}

impl IntoClusterIterator for [u8; 81] {
    fn cluster<'a>(&'a self, index: u8) -> ClusterIterator<'a> {
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

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    row: u8,
    col: u8,
}

impl Coordinate {
    fn as_linear(&self) -> usize {
        if self.row > 8 || self.col > 8 {
            panic!("Cannot linearize (row: {}, col: {}", self.row, self.col);
        }
        usize::from(self.row * 9 + self.col)
    }
}

#[cfg(test)]
mod test {
    use super::{Board, Coordinate, IntoClusterIterator, IntoColumnIterator, IntoRowIterator};

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
                assert_eq!(board.get(Coordinate { row, col }), row * 9 + col);
            }
        }
    }

    #[test]
    fn set() {
        let mut board = sequential_board();
        board.set(Coordinate { row: 4, col: 5 }, 1);

        for row in 0..9 {
            for col in 0..9 {
                if row == 4 && col == 5 {
                    assert_eq!(board.get(Coordinate { row, col }), 1);
                } else {
                    assert_eq!(board.get(Coordinate { row, col }), row * 9 + col);
                }
            }
        }
    }

    #[test]
    fn clear() {
        let mut board = sequential_board();
        board.clear(Coordinate { row: 4, col: 5 });

        for row in 0..9 {
            for col in 0..9 {
                if row == 4 && col == 5 {
                    assert_eq!(board.get(Coordinate { row, col }), 0);
                } else {
                    assert_eq!(board.get(Coordinate { row, col }), row * 9 + col);
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
                assert_eq!(board.get(Coordinate { row, col }), col * 9 + row);
            }
        }
    }

    #[test]
    fn mirror_columns() {
        let mut board = sequential_board();
        board.mirror_columns();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.get(Coordinate { row, col }), row * 9 + (8 - col));
            }
        }
    }

    #[test]
    fn mirror_rows() {
        let mut board = sequential_board();
        board.mirror_rows();

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(board.get(Coordinate { row, col }), (8 - row) * 9 + col);
            }
        }
    }

    #[test]
    fn swap_columns() {
        let mut board = sequential_board();
        board.swap_columns(0, 2);
        let mut expected = sequential_board();

        for row in 0..9 {
            for col in 0..9 {
                if col == 0 {
                    assert_eq!(
                        board.get(Coordinate { row, col }),
                        expected.get(Coordinate { row, col: col + 1 })
                    );
                } else if col == 1 {
                    assert_eq!(
                        board.get(Coordinate { row, col }),
                        expected.get(Coordinate { row, col: col - 1 })
                    );
                } else {
                    assert_eq!(
                        board.get(Coordinate { row, col }),
                        expected.get(Coordinate { row, col })
                    );
                }
            }
        }
    }

    #[test]
    fn swap_rows() {
        let mut board = sequential_board();
        board.swap_rows(0, 2);
        let mut expected = sequential_board();

        for row in 0..9 {
            for col in 0..9 {
                if row == 0 {
                    assert_eq!(
                        board.get(Coordinate { row, col }),
                        expected.get(Coordinate { row: row + 1, col })
                    );
                } else if row == 1 {
                    assert_eq!(
                        board.get(Coordinate { row, col }),
                        expected.get(Coordinate { row: row - 1, col })
                    );
                } else {
                    assert_eq!(
                        board.get(Coordinate { row, col }),
                        expected.get(Coordinate { row, col })
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
        let mut expected = sequential_board();

        for row in 0..9 {
            for col_index in 0..9 {
                let col = (col_index + 1) % 3;
                assert_eq!(
                    board.get(Coordinate { row, col }),
                    expected.get(Coordinate { row, col })
                );
            }
        }
    }

    #[test]
    fn swap_row_cluster() {
        let mut board = sequential_board();
        board.swap_row_cluster(2);
        board.swap_row_cluster(1);
        let mut expected = sequential_board();

        for row_index in 0..9 {
            for col in 0..9 {
                let row = (row_index + 1) % 3;
                assert_eq!(
                    board.get(Coordinate { row, col }),
                    expected.get(Coordinate { row, col })
                );
            }
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
    }
}

// swapRowClusters(pivot: number) {
//   let i1 = ((pivot + 1) % 3) * 3
//   let i2 = ((pivot + 2) % 3) * 3

//   this.swapRowsByIndex(i1, i2)
//   this.swapRowsByIndex(i1 + 1, i2 + 1)
//   this.swapRowsByIndex(i1 + 2, i2 + 2)
// }

// swapColumnClusters(pivot: number) {
//   let i1 = ((pivot + 1) % 3) * 3
//   let i2 = ((pivot + 2) % 3) * 3

//   this.swapColumnsByIndex(i1, i2)
//   this.swapColumnsByIndex(i1 + 1, i2 + 1)
//   this.swapColumnsByIndex(i1 + 2, i2 + 2)
// }

// private swapRowsByIndex(index1: number, index2: number) {
//   let temp = this.getRow(index1)
//   this.setRow(index1, this.getRow(index2))
//   this.setRow(index2, temp)
// }

// private swapColumnsByIndex(index1: number, index2: number) {
//   let temp = this.getColumn(index1)
//   this.setColumn(index1, this.getColumn(index2))
//   this.setColumn(index2, temp)
// }
// }

/*
import Coordinate from './Coordinate'

export default class Matrix {
  private board: number[]

  constructor(matrix?: Matrix) {
    if (matrix) {
      this.board = Array.from(matrix.board)
    } else {
      this.board = [
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
      ]
    }
  }

  from(board: number[]) {
    this.board = board
    return this
  }

  initialize() {
    this.board = [
      1, 2, 3, 4, 5, 6, 7, 8, 9,
      4, 5, 6, 7, 8, 9, 1, 2, 3,
      7, 8, 9, 1, 2, 3, 4, 5, 6,
      2, 3, 4, 5, 6, 7, 8, 9, 1,
      5, 6, 7, 8, 9, 1, 2, 3, 4,
      8, 9, 1, 2, 3, 4, 5, 6, 7,
      3, 4, 5, 6, 7, 8, 9, 1, 2,
      6, 7, 8, 9, 1, 2, 3, 4, 5,
      9, 1, 2, 3, 4, 5, 6, 7, 8,
    ]
  }

  getRow(index: number): number[] {
    return Array.from(Array(9), (v, i) => this.board[index * 9 + i])
  }

  setRow(index: number, row: number[]) {
    row.forEach((v, i) => this.board[index * 9 + i] = v)
  }

  getColumn(index: number): number[] {
    return Array.from(Array(9), (v, i) => this.board[i * 9 + index])
  }

  setColumn(index: number, column: number[]) {
    column.forEach((v, i) => this.board[i * 9 + index] = v)
  }

  getCluster(index: number): number[] {
    return Array.from(Array(9), (v, i) => this.board[i % 3 + Math.floor(i / 3) * 9 + (index % 3) * 3 + Math.floor(index / 3) * 27])
  }

  setCluster(index: number, cluster: number[]) {
    cluster.forEach((v, i) => this.board[i % 3 + Math.floor(i / 3) * 9 + (index % 3) * 3 + Math.floor(index / 3) * 27] = v)
  }

  getValue(index: Coordinate) {
    return this.board[index.row * 9 + index.column]
  }

  setValue(index: Coordinate, value: number) {
    this.board[index.row * 9 + index.column] = value
  }

  reverse() {
    this.board = this.board.reverse()
  }

  rotate() {
    this.board = Array.from(Array(9), (v, i) => this.getColumn(8 - i)).flat()
  }

  mirrowRows() {
    this.reverse()
    this.board = Array.from(Array(9), (v, i) => this.getRow(8 - i)).flat()
  }

  mirrorColumns() {
    this.board = Array.from(Array(9), (v, i) => this.getRow(8 - i)).flat()
  }

  swapRows(cluster: number, pivot: number) {
    this.swapRowsByIndex(((pivot + 1) % 3) + cluster * 3, ((pivot + 2) % 3) + cluster * 3)
  }

  swapColumns(cluster: number, pivot: number) {
    this.swapColumnsByIndex(((pivot + 1) % 3) + cluster * 3, ((pivot + 2) % 3) + cluster * 3)
  }

  swapRowClusters(pivot: number) {
    let i1 = ((pivot + 1) % 3) * 3
    let i2 = ((pivot + 2) % 3) * 3

    this.swapRowsByIndex(i1, i2)
    this.swapRowsByIndex(i1 + 1, i2 + 1)
    this.swapRowsByIndex(i1 + 2, i2 + 2)
  }

  swapColumnClusters(pivot: number) {
    let i1 = ((pivot + 1) % 3) * 3
    let i2 = ((pivot + 2) % 3) * 3

    this.swapColumnsByIndex(i1, i2)
    this.swapColumnsByIndex(i1 + 1, i2 + 1)
    this.swapColumnsByIndex(i1 + 2, i2 + 2)
  }

  private swapRowsByIndex(index1: number, index2: number) {
    let temp = this.getRow(index1)
    this.setRow(index1, this.getRow(index2))
    this.setRow(index2, temp)
  }

  private swapColumnsByIndex(index1: number, index2: number) {
    let temp = this.getColumn(index1)
    this.setColumn(index1, this.getColumn(index2))
    this.setColumn(index2, temp)
  }

  clone() {
    return new Matrix(this)
  }

  equals(other: Matrix) {
    for (let i = 0; i < this.board.length; i++) {
      if (this.board[i] !== other.board[i]) {
        return false;
      }
    }

    return true;
  }

  toString() {
    return '|' + this.board.reduce((s, v, i, a) => {
      if (i > 0) {
        if (i % 3 === 0) {
          s += ' |'
        }

        if (i % 9 === 0) {
          s += '\n|'
        }
      }

      return s + ' ' + a[i]
    }, '') + ' |'
  }

  print() {
    return this.board.reduce((s, v, i, a) => {
      if (i > 0) {
        return s + ',' + (a[i] > 0 ? a[i] : ' ')
      } else {
        return s + (a[i] > 0 ? a[i] : ' ')
      }
    }, '[') + ']'
  }
}

*/
