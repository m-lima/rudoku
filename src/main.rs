#![deny(warnings, clippy::pedantic, rust_2018_idioms)]
#![cfg_attr(all(test, nightly), feature(test))]
#![allow(dead_code)]

#[macro_use]
mod error;

mod game;
mod index;
mod options;

fn main() {
    let options = options::parse();

    match options {
        options::Options::Generate(options) => {
            let mut solved = options
                .template()
                .solve()
                .expect("The template given is unsolvable");

            let mut i = 0_u16;
            let mut puzzles = [None; 3];
            loop {
                if i & 0xF == 0 {
                    eprintln!("Pruning");
                    puzzles = solved.prune_per_gaps(options.max_difficulty());
                }

                shuffle_puzzle(&mut solved, &mut puzzles);
                print_puzzle(&solved, &puzzles);

                if options.count() > 0 {
                    if i == u16::max_value() {
                        i = 0;
                    } else {
                        i += 1;
                        if i == options.count() {
                            break;
                        }
                    }
                }
            }
        }
        options::Options::Solve(options) => {
            if let Some(solved) = options.puzzle().solve() {
                println!("{}", solved);
                println!("{:?}", solved);
            } else {
                println!("Puzzle is unsolvable");
            }
        }
        options::Options::Play(_) => {}
    }
}

fn shuffle_puzzle(solved: &mut game::Game, puzzles: &mut [Option<game::Game>; 3]) {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let seed = rng.gen::<u64>();
    *solved = solved.shuffle(seed);
    for puzzle in puzzles.iter_mut().flatten() {
        *puzzle = puzzle.shuffle(seed);
    }
}

fn print_puzzle(solved: &game::Game, puzzles: &[Option<game::Game>; 3]) {
    println!("Solved: [{:?}]", solved);
    if let Some(easy) = puzzles[0] {
        println!("Easy:   [{:?}]", easy);
    }
    if let Some(medium) = puzzles[1] {
        println!("Medium: [{:?}]", medium);
    }
    if let Some(hard) = puzzles[2] {
        println!("Hard:   [{:?}]", hard);
    }
}
