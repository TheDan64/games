use extra::rand::Randomizer;
use grader::{Difficulty, RedokuGrader};
use redoku::Redoku;
use solver::RedokuSolver;
use utils::{random_cell_value, read_u8_in_range};
use value::Value;

#[derive(Debug)]
pub struct RedokuGenerator {

}

impl RedokuGenerator {
    pub fn build(rand: &mut Randomizer, difficulty: Difficulty) -> Redoku {
        let mut redoku = Redoku::new(); // TODO: with_capacity based on difficulty
        let mut filled_cells = 0;

        loop {
            redoku.clear();
            filled_cells = 0;

            while filled_cells < 11 {
                let x = read_u8_in_range(rand, 0..9);
                let y = read_u8_in_range(rand, 0..9);

                if redoku[(x, y)].is_some() {
                    continue;
                }

                loop {
                    let value = random_cell_value(rand);

                    if redoku.place_if_valid(x, y, Some(value)) {
                        filled_cells += 1;
                        break;
                    }
                }

            };

            println!("Attempting to solve:\n{:?}", redoku);

            if let Some((sol, _)) = redoku.find_unique_solution() {
                println!("Found terminal pattern:\n{:?}", sol);
                break;
            };
        }

        println!("{:?}", redoku);

        redoku
    }
}

// #[test]
// fn test_build_tmp() {
//     let mut rand = Randomizer::new(0xBEEF);

//     let redoku = RedokuGenerator::build(&mut rand, Difficulty::Easy);

//     assert!(false);
// }
