use extra::rand::Randomizer;
use grader::{Difficulty, RedokuGrader};
use redoku::Redoku;
use solver::RedokuSolver;
use utils::{random_cell_value, read_u8_in_range};
// use value::Value;

pub trait RedokuBuilder {
    fn build(rand: &mut Randomizer, difficulty: Difficulty) -> Self where Self: Sized;
}

fn build_terminal_pattern(redoku: &mut Redoku, rand: &mut Randomizer) {
    let mut filled_cells;

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

        if let Some((sol, _)) = redoku.find_solution(false) {
            println!("Found terminal pattern:\n{:?}", sol);
            break;
        };
    }
}

struct Sequence {
    difficulty: Difficulty,
    pos: (u8, u8),
}

impl Iterator for Sequence {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<(u8, u8)> {
        if self.pos == (8, 8) {
            return None;
        }

        match self.difficulty {
            Difficulty::VeryEasy => {
                // Randomized
                Some((0, 0))
            },
            Difficulty::Easy => {
                // Randomized
                Some((0, 0))
            },
            Difficulty::Medium => {
                // Jumping one cell (c)

                Some((0, 0))
            },
            Difficulty::Hard => {
                // "S" path (b)

                Some((0, 0))
            },
            Difficulty::Evil => {
                // Left to Right then Top to Bottom (a)
                let (old_x, old_y) = self.pos;

                self.pos = if old_x == 8 {
                    (0, old_y + 1)
                } else {
                    (old_x + 1, old_y)
                };

                Some((old_x, old_y))
            },
        }
    }
}

fn build_sequence(difficulty: Difficulty) -> Sequence {
    Sequence {
        difficulty: difficulty,
        pos: (0, 0),
    }
}

impl RedokuBuilder for Redoku {
    fn build(rand: &mut Randomizer, difficulty: Difficulty) -> Redoku {
        let mut redoku = Redoku::with_capacity(match difficulty { // TODO: Tweak
            Difficulty::VeryEasy => 0,
            Difficulty::Easy => 0,
            Difficulty::Medium => 2,
            Difficulty::Hard => 13,
            Difficulty::Evil => 15,
        });

        build_terminal_pattern(&mut redoku, rand);

        println!("Terminal Pattern:\n{:?}", redoku);

        // TODO: Set all cells as "can dig"

        for (x, y) in build_sequence(difficulty) {//.cycle() {
            println!("{}, {}", x, y);
        }

        redoku
    }
}

#[test]
fn test_build_tmp() {
    let mut rand = Randomizer::new(0xBEEF);

    let redoku = Redoku::build(&mut rand, Difficulty::VeryEasy);
}
