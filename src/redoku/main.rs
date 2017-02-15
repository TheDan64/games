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

mod frontend;
mod generator;
mod grader;
#[macro_use]
mod redoku;
mod solver;
mod utils;
mod value;

use redoku::Redoku;
use extra::rand::Randomizer;
use frontend::event::Event;
use frontend::terminal::Terminal;
use std::io::{self, Read, Write};
// use termion::

struct Game {
    rand: Randomizer,
    redoku: Option<Redoku>,
}

impl Game {
    fn new() -> Game {
        Game {
            rand: Randomizer::new(!0x5AFEC0DE404),
            redoku: None,
        }
    }
}

enum State {
    MainMenu,
}

fn main() {
    // let mut game = Game::new();
    // let mut terminal = Terminal::new();
    // let mut state = State::MainMenu;
    // let stdin = io::stdin();
    // let stdin = stdin.lock();

    // while true {
    //     terminal.update(Event::MenuInit);
    // };

    println!("Hello, Redox!");

    // Test:
    // use utils::get_evil_redoku2;
    // use solver::RedokuSolver;

    // let redoku = get_evil_redoku2();
    use value::Value::*;
    use solver::RedokuSolver;

    let redoku = redoku![
        ?,?,?, ?,?,?, ?,?,?,
        ?,?,?, ?,?,?, ?,?,8,
        6,7,1, 4,5,8, 9,2,3,

        4,1,2, 3,9,6, 7,8,5,
        7,9,5, 1,8,2, 6,3,4,
        8,3,6, 5,7,4, 2,1,9,

        1,2,3, 7,4,5, 8,9,6,
        5,6,4, 8,2,9, 3,7,1,
        9,8,7, 6,3,1, 4,5,2,
    ];

    println!("Start:\n{:?}", redoku);

    println!("Solution:\n{:?}", redoku.find_solution(true));
}
