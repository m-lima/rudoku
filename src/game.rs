mod board;
mod ops;

use board::Board;

#[derive(Copy, Clone, Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(_difficulty: Difficulty) -> Self {
        let mut board = ops::solve(&Board::new()).expect("Could not solve an empty board");
        board.shuffle();
        Self { board }
    }

    #[inline]
    pub fn board(&self) -> &Board {
        &self.board
    }

    fn list_inconsistencies(&self) -> Vec<Cell> {
        let mut inconsistencies = Vec::new();
        for row in 0..9 {
            for col in 0..9 {
                let cell = Cell { row, col };
                if !ops::consistent(&self.board, cell) {
                    inconsistencies.push(cell);
                }
            }
        }
        inconsistencies
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
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

// Allowed because this is a test
#[allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use super::{Cell, Token};

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
}
