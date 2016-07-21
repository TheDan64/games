use redoku::{Difficulty, Redoku};

#[derive(Debug)]
pub struct RedokuGenerator {

}

impl RedokuGenerator {
    pub fn build(&self, difficulty: Difficulty) -> Redoku {
        let redoku = Redoku::new(difficulty);

        // Create a randomly generated valid start, pass it to solver?

        // Gets a Redoku from Solver

        // Does hole digging of Difficulty?

        redoku
    }
}
