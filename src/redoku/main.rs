//! Sudoku clone by Daniel Kolsoi
//! Dedicated to the Redox community
//! Approach and methodology is mostly based on the paper:
//! http://zhangroup.aporc.org/images/files/Paper_3485.pdf

#![feature(test)]
// For the redoku! macro. Shouldn't be needed with
// https://github.com/rust-lang/rust/issues/22552 fixed
// or increased
#![recursion_limit = "86"]

#[macro_use]
extern crate extra;
// extern crate rand;
extern crate termion;
extern crate test;

mod generator;
mod grader;
#[macro_use]
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
        // TODO: format!("{:p}", rand) as digit to seed randomizer

        Game {
            rand: Randomizer::new(!0x5AFEC0DE404)
        }
    }
}

struct Frame {
    // GridValue = Value | ValueSet
    // [Optional<GridValue>; 81]
    // cursor_pointed_at: u8
}

// IDEA: Frontend A or B gets passed a Option<Fame>
struct Terminal {

}

fn main() {

}
