use clap::Clap;

use crate::error;
use crate::game;

static EMPTY_BOARD: &str = "0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0";

pub fn parse() -> Options {
    Options::parse()
}

#[derive(Clap, Debug)]
#[clap(name = "Rudoku")]
pub enum Options {
    /// Generate new puzzles
    Generate(Generate),
    /// Solve a given puzzle
    Solve(Solve),
    /// Play Sudoku
    Play(Play),
}

#[derive(Clap, Debug)]
pub struct Generate {
    /// A base template to work from
    #[clap(short, long, default_value = EMPTY_BOARD, parse(try_from_str = to_game))]
    template: game::Game,
    /// The max difficulty to try for
    #[clap(short ="d", long, default_value = "m", parse(try_from_str = to_difficulty))]
    max_difficulty: game::Difficulty,
    /// Number of puzzles to generate (0 for indefinite)
    #[clap(short, long, default_value = "1")]
    count: u16,
}

#[derive(Clap, Debug)]
pub struct Solve {
    /// Puzzle to be solved
    #[clap(short, long, parse(try_from_str = to_game))]
    puzzle: game::Game,
}

#[derive(Clap, Debug)]
pub struct Play {
    /// Puzzle to play
    #[clap(short, long, default_value = EMPTY_BOARD, parse(try_from_str = to_game))]
    puzzle: game::Game,
    /// Difficulty to play in
    #[clap(short, long, default_value = "m", parse(try_from_str = to_difficulty))]
    difficulty: game::Difficulty,
}

impl Generate {
    pub fn template(&self) -> game::Game {
        self.template
    }
    pub fn max_difficulty(&self) -> game::Difficulty {
        self.max_difficulty
    }
    pub fn count(&self) -> u16 {
        self.count
    }
}

impl Solve {
    pub fn puzzle(&self) -> game::Game {
        self.puzzle
    }
}

// Allowed because it is so much clearer
#[allow(clippy::filter_map)]
fn to_game(value: &str) -> Result<game::Game, error::Error> {
    let clean_value = value
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| (c as u8) - b'0')
        .collect::<Vec<u8>>();

    if clean_value.len() == 81 {
        let mut board = [0; 81];
        board.copy_from_slice(&clean_value[0..81]);
        Ok(game::Game::from(board))
    } else {
        error!("board must be 9x9")
    }
}

fn to_difficulty(value: &str) -> Result<game::Difficulty, error::Error> {
    match value.to_uppercase().as_str() {
        "EASY" | "E" => Ok(game::Difficulty::Easy),
        "MEDIUM" | "M" => Ok(game::Difficulty::Medium),
        "HARD" | "H" => Ok(game::Difficulty::Hard),
        _ => error!("possible values are [easy, medium, hard]",),
    }
}
