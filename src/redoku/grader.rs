use redoku::{CellValue, Redoku};
use redoku::CellValue::*;
use std::cmp::{max, min};
#[cfg(test)]
use test::Bencher;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    Evil,
}

fn try_row_col_block_elimination(redoku: &mut Redoku) -> bool {
    let mut success = false;

    // TODO: Randomize the range of values for potentially better results
    // as doing x and y incrementally will favor some paths over others
    for x in 0..9 {
        for y in 0..9 {
            if redoku[(x, y)].is_some() {
                continue;
            }

            let values = redoku.calculate_impossible_values(x, y);

            let (count, sum) = values.iter().fold((0, 0), |(a, b), &v| (a + 1, b + v as usize));

            // Place the missing value determined from 45 (sum(1...9))
            if count == 8 {
                assert!(redoku.place_if_valid(x, y, Some(CellValue::from_usize(45 - sum))));

                success = true;
            }
        }
    }

    success
}

fn try_lone_ranger(redoku: &mut Redoku) -> bool {
    let mut success = false;

    // TODO: Randomize the range of values for potentially better results
    // as doing x and y incrementally will favor some paths over others
    for x in 0..9 {
        for y in 0..9 {
            if redoku[(x, y)].is_some() {
                continue;
            }

            let mut row_values = redoku.calculate_possible_values(x, y);
            let mut column_values = row_values.clone();
            let mut block_values = row_values.clone();

            println!("{},{} starting values: {:?}", x, y, row_values);

            let (block_x, block_y) = (x / 3, y / 3);

            for i in 0..9 {
                let (row_x, row_y) = (x, i);
                let (column_x, column_y) = (i, y);
                let (block_x, block_y) = (block_x * 3 + i % 3, block_y * 3 + i / 3);

                if (row_x, row_y) != (x, y) && redoku[(row_x, row_y)].is_none() {
                    row_values = &row_values - &redoku.calculate_possible_values(row_x, row_y);
                    println!("Row: {:?}", row_values);
                }

                if (column_x, column_y) != (x, y) && redoku[(column_x, column_y)].is_none() {
                    column_values = &column_values - &redoku.calculate_possible_values(column_x, column_y);
                    println!("Column: {:?}", column_values);
                }

                if (block_x, block_y) != (x, y) && redoku[(block_x, block_y)].is_none() {
                    block_values = &block_values - &redoku.calculate_possible_values(block_x, block_y);
                    println!("Block: {:?}", block_values);
                }
            }

            if row_values.len() == 1 {
                let value = row_values.iter().next().unwrap();

                if redoku.place_if_valid(x, y, Some(*value)) {
                    success = true;
                    break;
                }
            }

            if column_values.len() == 1 {
                let value = column_values.iter().next().unwrap();

                if redoku.place_if_valid(x, y, Some(*value)) {
                    success = true;
                    break;
                }
            }

            if block_values.len() == 1 {
                let value = block_values.iter().next().unwrap();

                if redoku.place_if_valid(x, y, Some(*value)) {
                    success = true;
                }
            }
        }
    }

    success
}

fn score_cell_total_count(redoku: &Redoku) -> f32 {
    // Difficulty | Givens  | Scores
    // Very Easy  |    > 50 |   1
    // Easy       | 36 - 49 |   2
    // Medium     | 32 - 35 |   3
    // Hard       | 28 - 31 |   4
    // Evil       | 22 - 27 |   5

    match 81 - redoku.empty_cells() {
        givens if givens >= 50 => 1.0,
        36...50 => 2.0,
        32...36 => 3.0,
        28...32 => 4.0,
        22...28 => 5.0,
        _ => panic!("No evaluation metric for number of givens under 22"),
    }
}

fn score_cell_row_column_count(redoku: &Redoku) -> f32 {
    // Difficulty | Lower bound of    |
    //            | givens in row/col | Scores
    // Very Easy  |        5          |   1
    // Easy       |        4          |   2
    // Medium     |        3          |   3
    // Hard       |        2          |   4
    // Evil       |        0          |   5

    let mut min_len = 9;

    for i in 0..9 {
        min_len = min(min_len, redoku.row_values(i).len());
        min_len = min(min_len, redoku.column_values(i).len());

        if min_len == 0 {
            break;
        }
    }

    match min_len {
        0 => 5.0,
        1 => 4.0, // REVIEW: Is this just eq to 2?
        2 => 4.0,
        3 => 3.0,
        4 => 2.0,
        5 => 1.0,
        _ => 1.0, // REVIEW: Is this correct? Possible in a valid inital Redoku?
    }
}

fn score_human_solving_techniques(redoku: &Redoku) -> f32 {
    // Technique                               | Score
    // Row, Column, and Block Elimination      |   1
    // Lone rangers in Block/Column/Row        |   2
    // Twins in Block/Column/Row               |   3
    // Triplets in Block/Column/Row            |   4
    // Brute-force Elimination                 |   5

    let mut max_score = 0;
    let mut redoku = redoku.clone();

    loop {
        let rcb_elimination = try_row_col_block_elimination(&mut redoku);

        if rcb_elimination {
            max_score = max(max_score, 1);

            if redoku.empty_cells() == 0 {
                break;
            }
        }

        let lone_ranger = try_lone_ranger(&mut redoku);

        if lone_ranger {
            max_score = max(max_score, 2);

            if redoku.empty_cells() == 0 {
                break;
            }
        }

        // TODO: More

        // If no other method worked, need to brute force to solve. Instead,
        // assuming there is a valid solution means we can skip doing so.
        if !rcb_elimination && !lone_ranger && true && true && true {
            max_score = 5;
            break;
        }
    }

    // Seems to be no max() for floats due to no full ordering
    max_score as f32
}

// Difficulty | Enum. search times | Score
// Very Easy  |      100 <         |   1
// Easy       |      100 -    999  |   2
// Medium     |    1,000 -  9,999  |   3
// Hard       |   10,000 - 99,999  |   4
// Evil       |  100,000 >         |   5

pub trait RedokuGrader {
    fn grade_difficulty(&self) -> Difficulty;
}

impl RedokuGrader for Redoku {
    fn grade_difficulty(&self) -> Difficulty {
        let mut total_score = 0.4 * score_cell_total_count(&self);

        total_score += 0.2 * score_cell_row_column_count(&self);
        total_score += 0.2 * score_human_solving_techniques(&self);

        // Enumerating Search

        match total_score.round() {
            1.0 => Difficulty::VeryEasy,
            2.0 => Difficulty::Easy,
            3.0 => Difficulty::Medium,
            4.0 => Difficulty::Hard,
            5.0 => Difficulty::Evil,
            _ => unreachable!("Grading metric failure"),
        }
    }
}

#[bench]
fn test_column_row_block_elimination(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku.place_if_valid(0, 0, Some(Four));
    redoku.place_if_valid(0, 1, Some(Two));
    redoku.place_if_valid(0, 2, Some(One));
    // Empty (0, 3)
    redoku.place_if_valid(0, 4, Some(Six));
    redoku.place_if_valid(0, 5, Some(Seven));
    // Empty (0, 6)
    redoku.place_if_valid(0, 7, Some(Nine));
    redoku.place_if_valid(0, 8, Some(Five));

    redoku.place_if_valid(1, 3, Some(Five));
    redoku.place_if_valid(1, 4, Some(One));
    // Empty (1, 5)

    redoku.place_if_valid(2, 3, Some(Eight));
    redoku.place_if_valid(2, 4, Some(Four));
    redoku.place_if_valid(2, 5, Some(Nine));

    assert!(redoku.empty_cells() == 69);
    b.iter(|| {
        let mut cloned = redoku.clone();

        assert!(try_row_col_block_elimination(&mut cloned));
        assert!(cloned.empty_cells() == 66);
        assert!(cloned[(0, 3)] == Some(Three));
        assert!(cloned[(0, 6)] == Some(Eight));
        assert!(cloned[(1, 5)] == Some(Two));
    });
}

#[bench]
fn test_lone_ranger(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku.place_if_valid(5, 0, Some(One));

    // Empty (6, 0)
    // Empty (6, 1)
    redoku.place_if_valid(6, 2, Some(Two));

    // Empty (7, 0)
    redoku.place_if_valid(7, 1, Some(Six));
    redoku.place_if_valid(7, 2, Some(Eight));

    // Empty (8, 0)
    redoku.place_if_valid(8, 1, Some(Nine));
    redoku.place_if_valid(8, 2, Some(Seven));

    assert!(redoku.empty_cells() == 75);
    b.iter(|| {
        let mut cloned = redoku.clone();

        assert!(try_lone_ranger(&mut cloned));
        assert!(cloned.empty_cells() == 74);
        assert!(cloned[(6, 1)] == Some(One));
    });
}

#[test]
fn test_grade_very_easy_redoku() {
    use utils;

    let redoku = utils::get_very_easy_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::VeryEasy);
}

#[test]
fn test_grade_easy_redoku() {
    use utils;

    let redoku = utils::get_easy_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Easy);
}

#[test]
fn test_grade_medium_redoku() {
    use utils;

    let redoku = utils::get_medium_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Medium);
}

#[test]
fn test_grade_hard_redoku() {
    use utils;

    let redoku = utils::get_hard_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Hard);
}

#[test]
fn test_grade_evil_redoku() {
    use utils;

    let redoku = utils::get_evil_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Evil);
}
