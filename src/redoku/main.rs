//! Sudoku clone by Daniel Kolsoi
//! Dedicated to the Redox community
//! Approach and methodology is mostly based on the paper:
//! http://zhangroup.aporc.org/images/files/Paper_3485.pdf

#![feature(test)]

#[macro_use]
extern crate extra;
// extern crate rand;
extern crate termion;
extern crate test;

mod generator;
mod grader;
mod redoku;
mod solver;
mod utils;
mod value;

use extra::rand::Randomizer;

struct Game {
    rand: Randomizer
}

impl Game {
    fn new() -> Game {
        Game {
            rand: Randomizer::new(!0x5AFEC0DE404)
        }
    }
}

// fn render() {

// }

fn main() {
    println!("Hello, Redox!");

    // Test:
    // use utils::get_evil_redoku2;
    // use solver::RedokuSolver;

    // let redoku = get_evil_redoku2();
    use value::Value::*;
    use solver::RedokuSolver;

    let mut redoku = redoku::Redoku::new();

    redoku.place_if_valid(2, 1, Some(Five));
    redoku.place_if_valid(2, 7, Some(Seven));

    redoku.place_if_valid(3, 4, Some(One));
    redoku.place_if_valid(3, 8, Some(Nine));

    redoku.place_if_valid(4, 3, Some(Seven));
    redoku.place_if_valid(4, 4, Some(Eight));

    redoku.place_if_valid(5, 3, Some(Four));
    redoku.place_if_valid(5, 5, Some(Five));
    redoku.place_if_valid(5, 7, Some(Three));

    redoku.place_if_valid(6, 4, Some(Nine));

    redoku.place_if_valid(7, 8, Some(Six));

    println!("Start:\n{:?}", redoku);

    println!("Solution:\n{:?}", redoku.find_solution(false));
}
