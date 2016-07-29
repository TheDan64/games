use extra::rand::Randomizer;
use grader::{Difficulty, RedokuGrader};
use redoku::{CellValue, Redoku};
use solver::RedokuSolver;

#[derive(Debug)]
pub struct RedokuGenerator {

}

impl RedokuGenerator {
    pub fn build(&self, difficulty: Difficulty) -> Redoku {
        let redoku = Redoku::new();

        // Create a randomly generated valid start, pass it to solver?

        // Gets a Redoku from Solver

        // Does hole digging of Difficulty?

        redoku
    }
}
