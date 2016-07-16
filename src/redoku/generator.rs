use redoku::{Difficulty, Redoku};

#[derive(Debug)]
pub struct RedokuGenerator {

}

impl RedokuGenerator {
    pub fn build(&self, difficulty: Difficulty) -> Redoku {
        let redoku = Redoku::new();

        redoku
    }
}
