//! Sudoku clone by Daniel Kolsoi
//! Dedicated to the Redox community
//! Approach and methodology is based on the paper:
//! http://zhangroup.aporc.org/images/files/Paper_3485.pdf

extern crate termion;

mod generator;
mod grader;
mod redoku;
mod solver;

fn main() {
    println!("Hello, Redoku!");
}
