use extra::rand::Randomizer;
use grader::{Difficulty, RedokuGrader};
use redoku::Redoku;
use solver::RedokuSolver;
use utils::{random_cell_value, read_u8_in_range};
#[cfg(test)]
use value::{Value, ValueSet};

pub trait RedokuBuilder {
    fn build(difficulty: Difficulty, rand: &mut Randomizer) -> Self where Self: Sized;
}

fn build_terminal_pattern(mut redoku: &mut Redoku, rand: &mut Randomizer) {
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

        if let Some((sol, _)) = redoku.find_solution(false) {
            *redoku = sol;
            break;
        };
    }
}

fn shuffle_redoku(redoku: &mut Redoku, rand: &mut Randomizer) {

}

struct Sequence<'a> {
    difficulty: Difficulty,
    pos: (u8, u8), // Option<>?
    rand: &'a mut Randomizer,
}

impl<'a> Iterator for Sequence<'a> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<(u8, u8)> {
        if self.pos.0 > 8 || self.pos.1 > 8 {
            return None;
        }

        match self.difficulty {
            Difficulty::VeryEasy |
            Difficulty::Easy => {
                // Randomized
                // TODO: Will need a way to break out inf loop if it gets stuck
                Some((read_u8_in_range(&mut self.rand, 0..9), read_u8_in_range(&mut self.rand, 0..9)))
            },
            Difficulty::Medium => {
                // Jumping one cell
                let (old_x, old_y) = self.pos;

                self.pos = if (old_x == 1 || old_x == 8) && old_y % 2 != old_x / 8 {
                    (old_x - 1, old_y + 1)
                } else if (old_x == 0 || old_x == 7) && old_y % 2 != old_x / 7 {
                    (old_x + 1, old_y + 1)
                } else if old_y % 2 == 0 {
                    (old_x + 2, old_y)
                } else {
                    (old_x - 2, old_y)
                };

                if self.pos == (7, 9) {
                    self.pos = (1, 0);
                }

                Some((old_x, old_y))
            },
            Difficulty::Hard => {
                // "S" path
                let (old_x, old_y) = self.pos;

                self.pos = if (old_x == 0 || old_x == 8) && old_y % 2 != old_x / 8 {
                    (old_x, old_y + 1)
                } else if old_y % 2 == 0 {
                    (old_x + 1, old_y)
                } else {
                    (old_x - 1, old_y)
                };

                Some((old_x, old_y))
            },
            Difficulty::Evil => {
                // Left to Right then Top to Bottom
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

impl<'a> Sequence<'a> {
    fn new(difficulty: Difficulty, rand: &'a mut Randomizer) -> Self {
        Sequence {
            difficulty: difficulty,
            pos: (0, 0),
            rand: rand,
        }
    }
}

impl RedokuBuilder for Redoku {
    fn build(difficulty: Difficulty, rand: &mut Randomizer) -> Redoku {
        let mut redoku = Redoku::with_capacity(match difficulty { // TODO: Tweak
            Difficulty::VeryEasy => 0,
            Difficulty::Easy => 0,
            Difficulty::Medium => 2,
            Difficulty::Hard => 13,
            Difficulty::Evil => 15,
        });

        build_terminal_pattern(&mut redoku, rand);

        println!("\nTerminal Pattern:\n{:?}", redoku);

        // REVIEW: What about Medium which is only 41?

        let (total_givens, lower_bound_row_col_givens) = match difficulty {
            Difficulty::VeryEasy => (read_u8_in_range(rand, 50..61), 5),
            Difficulty::Easy => (read_u8_in_range(rand, 36..50), 4),
            Difficulty::Medium => (read_u8_in_range(rand, 32..36), 3),
            Difficulty::Hard => (read_u8_in_range(rand, 28..32), 2),
            Difficulty::Evil => (read_u8_in_range(rand, 22..28), 0),
        };

        println!("Total givens: {}", total_givens);

        let mut completed_digs = 0;

        for (x, y) in Sequence::new(difficulty, rand) {
            print!("{}, {}", x, y);
            // Check restrictions
            if 81 - completed_digs == total_givens {
                println!("reached max givens");
                break;
            }

            // TODO: Ensure cell is != None, which can be true for the random sequences

            if redoku.row_values(y).len() == lower_bound_row_col_givens {
                println!(" Hit lower bound in row {}. Skipped.", y);
                continue;
            }

            if redoku.column_values(x).len() == lower_bound_row_col_givens {
                println!(" Hit lower bound in column {}. Skipped.", x);
                continue;
            }

            let original_value = redoku[(x, y)];

            redoku.place_if_valid(x, y, None);


            if !redoku.has_solution(true) {
                println!("{:?}", redoku);
                redoku.place_if_valid(x, y, original_value);
                println!(" Skipped 3");

                continue;
            }

            completed_digs += 1;

            println!("");
            // println!("{:?}", redoku);
        }

        println!("Final givens: {}", 81 - redoku.empty_cells());
        println!("Final redoku:\n{:?}", redoku);

        if 81 - completed_digs > total_givens {
            // Bad!
            // panic!("Ayye {} > {}", 81 - completed_digs, total_givens);
        }

        shuffle_redoku(&mut redoku, rand);

        redoku
    }
}

#[test]
fn test_build_very_easy() {
    let mut rand = Randomizer::new(!0xADABCDEAD);

    let redoku = Redoku::build(Difficulty::VeryEasy, &mut rand);

    let filled_cells = 81 - redoku.empty_cells();

    assert!(50 <= filled_cells && filled_cells < 61);
}

#[test]
fn test_build_easy() {
    let mut rand = Randomizer::new(!0xADABCDEAD);

    let redoku = Redoku::build(Difficulty::Easy, &mut rand);

    let filled_cells = 81 - redoku.empty_cells();

    assert!(36 <= filled_cells && filled_cells < 50);
}

#[test]
fn test_build_medium() {
    let mut rand = Randomizer::new(!0xADABCDEAD);

    let redoku = Redoku::build(Difficulty::Medium, &mut rand);

    let filled_cells = 81 - redoku.empty_cells();

    assert!(32 <= filled_cells && filled_cells < 36);
}

#[test]
fn test_build_hard() {
    let mut rand = Randomizer::new(!0xADABCDEAD); // 0xE15495FADDD -> 47 instead of 30
                                                  // 0xE15495FAEFEBBBDD -> 39 instead of 29

    let redoku = Redoku::build(Difficulty::Hard, &mut rand);

    let filled_cells = 81 - redoku.empty_cells();

    assert!(28 <= filled_cells && filled_cells < 32);
}

#[test]
fn test_build_evil() {
    let mut rand = Randomizer::new(!0xADABCDEAD);

    let redoku = Redoku::build(Difficulty::Evil, &mut rand);

    let filled_cells = 81 - redoku.empty_cells();

    assert!(22 <= filled_cells && filled_cells < 28);
}

#[test]
fn test_sequence_easy() {
    let mut rand = Randomizer::new(0xBEEF);
    let mut x_set = ValueSet::new(0);
    let mut y_set = ValueSet::new(0);

    for (x, y) in Sequence::new(Difficulty::Easy, &mut rand) {
        x_set.insert(x.into());
        y_set.insert(y.into());

        if x_set.is_full() && y_set.is_full() {
            break;
        }
    }
}

#[test]
fn test_sequence_medium() { // FIXME
    let mut count = 0;
    let mut rand = Randomizer::new(0xBEEF);

    for (x, y) in Sequence::new(Difficulty::Medium, &mut rand) {
        println!("{:?}", (x, y));
        // assert!(x < 9 && y < 9 && x + y * 9 == count);

        count += 1;
    }

    assert!(count == 41);
}

#[test]
fn test_sequence_hard() { // FIXME
    let mut count = 0;
    let mut rand = Randomizer::new(0xBEEF);

    for (x, y) in Sequence::new(Difficulty::Hard, &mut rand) {
        println!("{:?}", (x, y));
        // assert!(x < 9 && y < 9 && x + y * 9 == count);

        count += 1;
    }

    assert!(count == 81);
}

#[test]
fn test_sequence_evil() {
    let mut count = 0;
    let mut rand = Randomizer::new(0xBEEF);

    for (x, y) in Sequence::new(Difficulty::Evil, &mut rand) {
        assert!(x < 9 && y < 9 && x + y * 9 == count);

        count += 1;
    }

    assert!(count == 81);
}
