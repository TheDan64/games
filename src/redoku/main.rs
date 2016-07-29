//! Sudoku clone by Daniel Kolsoi
//! Dedicated to the Redox community
//! Approach and methodology is based on the paper:
//! http://zhangroup.aporc.org/images/files/Paper_3485.pdf

#![feature(test)]

extern crate termion;
extern crate extra;
extern crate test;

mod generator;
mod grader;
mod redoku;
mod solver;
mod utils;

use extra::rand::Randomizer;

struct Game {
    rand: Randomizer
}

fn render() {

}

fn main() {
    println!("Hello, Redoku!");

    loop {
        // Check for user input

        // Update Game State

        // Render
    }
}
