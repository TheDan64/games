use redoku::{Grid, Redoku};
use std::cmp::{max, min};
use value::Value::*;
use value::{Value, ValueSet};
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

            let mut values = redoku.calculate_impossible_values(x, y);
            // let count = values.len();
            // let sum: usize = values.iter().sum();

            let (count, sum) = values.fold((0, 0), |(a, b), v| (a + 1, b + v as u8));

            // Place the missing value determined from 36 (sum(0...8))
            if count == 8 {
                assert!(redoku.place_if_valid(x, y, Some(Value::from_u8(36 - sum))));

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

            let (block_x, block_y) = (x / 3, y / 3);

            for i in 0..9 {
                let (row_x, row_y) = (x, i);
                let (column_x, column_y) = (i, y);
                let (block_x, block_y) = (block_x * 3 + i % 3, block_y * 3 + i / 3);

                if (row_x, row_y) != (x, y) && redoku[(row_x, row_y)].is_none() {
                    row_values -= redoku.calculate_possible_values(row_x, row_y);
                }

                if (column_x, column_y) != (x, y) && redoku[(column_x, column_y)].is_none() {
                    column_values -= redoku.calculate_possible_values(column_x, column_y);
                }

                if (block_x, block_y) != (x, y) && redoku[(block_x, block_y)].is_none() {
                    block_values -= redoku.calculate_possible_values(block_x, block_y);
                }
            }

            if row_values.len() == 1 && redoku.place_if_valid(x, y, row_values.next()) {
                success = true;
                break;
            }

            if column_values.len() == 1 && redoku.place_if_valid(x, y, column_values.next()) {
                success = true;
                break;
            }

            if block_values.len() == 1 && redoku.place_if_valid(x, y, block_values.next()) {
                success = true;
            }
        }
    }

    success
}

fn try_look_for_twins_triplets(redoku: &mut Redoku) -> (bool, bool) {
    let mut twins = false;
    let mut triplets = false;

    // TODO: Randomize the range of values for potentially better results
    // as doing x and y incrementally will favor some paths over others
    for x in 0..9 {
        for y in 0..9 {
            if redoku[(x, y)].is_some() {
                continue;
            }

            let mut row_values = redoku.calculate_possible_values(x, y);

            let len = row_values.len();

            if len < 2 || len > 3 {
                continue;
            }

            let mut column_values = row_values;
            let mut block_values = row_values;

            let (block_grid_x, block_grid_y) = (x / 3, y / 3);
            // TODO: maybe define these outside to loop an reset values?
            let mut row_values_count = 1;
            let mut column_values_count = 1;
            let mut block_values_count = 1;

            let mut last_column = x;
            let mut last_row = y;
            let mut last_block = (block_grid_x, block_grid_y);

            for i in 0..9 {
                let (row_x, row_y) = (i, y);

                if (row_x, row_y) != (x, y) && redoku[(row_x, row_y)].is_none() {
                    let current_values = redoku.calculate_possible_values(row_x, row_y);

                    if current_values <= row_values && current_values.len() == 2 {
                        row_values_count += 1;
                        last_row = row_y;
                    } else if current_values > row_values && current_values.len() == 3 {
                        row_values = current_values;
                        row_values_count += 1;

                        last_row = row_x;
                    }
                }
            }

            if row_values_count == 2 {
                redoku.insert_temporary_values(Grid::Row(last_row), row_values);
                twins = true;
            } else if row_values_count == 3 {
                redoku.insert_temporary_values(Grid::Row(last_row), row_values);
                triplets = true;
            }

            for i in 0..9 {
                let (column_x, column_y) = (x, i);

                if (column_x, column_y) != (x, y) && redoku[(column_x, column_y)].is_none() {
                    let current_values = redoku.calculate_possible_values(column_x, column_y);

                    if current_values <= column_values && current_values.len() == 2 {
                        column_values_count += 1;
                        last_column = column_x;
                    } else if current_values > column_values && current_values.len() == 3 {
                        column_values = current_values;
                        column_values_count += 1;

                        last_column = column_y;
                    }
                }
            }

            if column_values_count == 2 {
                redoku.insert_temporary_values(Grid::Column(last_column), column_values);
                twins = true;
            } else if column_values_count == 3 {
                redoku.insert_temporary_values(Grid::Column(last_column), column_values);
                triplets = true;
            }

            for i in 0..9 {
                let (block_x, block_y) = (block_grid_x * 3 + i % 3, block_grid_y * 3 + i / 3);

                if (block_x, block_y) != (x, y) && redoku[(block_x, block_y)].is_none() {
                    let current_values = redoku.calculate_possible_values(block_x, block_y);

                    if current_values <= block_values && current_values.len() == 2 {
                        block_values_count += 1;
                    } else if current_values > block_values && current_values.len() == 3 {
                        block_values = current_values;
                        block_values_count += 1;
                    }
                }
            }

            if block_values_count == 2 {
                let (block_x, block_y) = last_block;

                redoku.insert_temporary_values(Grid::Block(3 * block_y + block_x), block_values);
                twins = true;
            } else if block_values_count == 3 {
                let (block_x, block_y) = last_block;

                redoku.insert_temporary_values(Grid::Block(3 * block_y + block_x), block_values);
                triplets = true;
            }
        }
    }

    (twins, triplets)
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

        let (twins, triplets) = try_look_for_twins_triplets(&mut redoku);

        if twins {
            max_score = max(max_score, 3);

            if redoku.empty_cells() == 0 {
                break;
            }
        }

        if triplets {
            max_score = max(max_score, 4);

            if redoku.empty_cells() == 0 {
                break;
            }
        }

        // If no other method worked, need to brute force to solve. Instead,
        // (assuming there is a valid solution means) we can skip doing so.
        if !rcb_elimination && !lone_ranger && !twins && !triplets {
            if redoku.temporary_values() > 0 {
                redoku.remove_temporary_values();

                continue;
            }

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

    assert!(redoku.place_if_valid(0, 1, Some(Six)));
    assert!(redoku.place_if_valid(0, 3, Some(Four)));
    assert!(redoku.place_if_valid(0, 4, Some(Five)));
    assert!(redoku.place_if_valid(0, 7, Some(Eight)));

    assert!(redoku.place_if_valid(1, 0, Some(Three)));
    assert!(redoku.place_if_valid(1, 4, Some(Six)));
    assert!(redoku.place_if_valid(1, 5, Some(Two)));
    assert!(redoku.place_if_valid(1, 7, Some(Five)));
    assert!(redoku.place_if_valid(1, 8, Some(Nine)));

    assert!(redoku.place_if_valid(2, 0, Some(Four)));
    assert!(redoku.place_if_valid(2, 2, Some(One)));
    assert!(redoku.place_if_valid(2, 3, Some(Nine)));
    assert!(redoku.place_if_valid(2, 7, Some(Seven)));

    assert!(redoku.place_if_valid(3, 5, Some(Five)));
    assert!(redoku.place_if_valid(3, 6, Some(Two)));
    assert!(redoku.place_if_valid(3, 7, Some(Nine)));

    assert!(redoku.place_if_valid(4, 2, Some(Two)));
    assert!(redoku.place_if_valid(4, 3, Some(Eight)));
    assert!(redoku.place_if_valid(4, 5, Some(Six)));
    assert!(redoku.place_if_valid(4, 6, Some(One)));

    assert!(redoku.place_if_valid(5, 1, Some(Eight)));
    assert!(redoku.place_if_valid(5, 2, Some(Seven)));
    assert!(redoku.place_if_valid(5, 3, Some(Three)));

    assert!(redoku.place_if_valid(6, 1, Some(Two)));
    assert!(redoku.place_if_valid(6, 5, Some(Four)));
    assert!(redoku.place_if_valid(6, 6, Some(Eight)));
    assert!(redoku.place_if_valid(6, 8, Some(Three)));

    assert!(redoku.place_if_valid(7, 0, Some(Nine)));
    assert!(redoku.place_if_valid(7, 1, Some(One)));
    assert!(redoku.place_if_valid(7, 3, Some(Five)));
    assert!(redoku.place_if_valid(7, 4, Some(Eight)));
    assert!(redoku.place_if_valid(7, 8, Some(Four)));

    assert!(redoku.place_if_valid(8, 1, Some(Four)));
    assert!(redoku.place_if_valid(8, 4, Some(Seven)));
    assert!(redoku.place_if_valid(8, 5, Some(One)));
    assert!(redoku.place_if_valid(8, 7, Some(Six)));

    assert!(redoku.empty_cells() == 45);
    b.iter(|| {
        let mut cloned = redoku.clone();

        assert!(try_row_col_block_elimination(&mut cloned));
        assert!(cloned.empty_cells() == 13);

        assert!(try_row_col_block_elimination(&mut cloned));
        assert!(cloned.empty_cells() == 2);

        assert!(try_row_col_block_elimination(&mut cloned));
        assert!(cloned.empty_cells() == 0);
    });
}

#[bench]
fn test_lone_ranger(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    assert!(redoku.place_if_valid(0, 0, Some(Four)));
    assert!(redoku.place_if_valid(0, 2, Some(Five)));
    assert!(redoku.place_if_valid(0, 3, Some(Seven)));
    assert!(redoku.place_if_valid(0, 4, Some(Six)));
    assert!(redoku.place_if_valid(0, 5, Some(Three)));
    assert!(redoku.place_if_valid(0, 7, Some(Nine)));
    assert!(redoku.place_if_valid(0, 8, Some(Two)));

    assert!(redoku.place_if_valid(1, 5, Some(Eight)));
    assert!(redoku.place_if_valid(1, 6, Some(Five)));

    assert!(redoku.place_if_valid(2, 0, Some(Nine)));
    assert!(redoku.place_if_valid(2, 1, Some(Seven)));
    assert!(redoku.place_if_valid(2, 2, Some(Eight)));
    assert!(redoku.place_if_valid(2, 3, Some(Five)));
    assert!(redoku.place_if_valid(2, 4, Some(Four)));
    assert!(redoku.place_if_valid(2, 6, Some(Six)));

    assert!(redoku.place_if_valid(3, 0, Some(Eight)));
    assert!(redoku.place_if_valid(3, 1, Some(Four)));
    assert!(redoku.place_if_valid(3, 4, Some(Two)));
    assert!(redoku.place_if_valid(3, 6, Some(Nine)));

    assert!(redoku.place_if_valid(4, 0, Some(Five)));
    assert!(redoku.place_if_valid(4, 3, Some(Six)));
    assert!(redoku.place_if_valid(4, 5, Some(Seven)));
    assert!(redoku.place_if_valid(4, 6, Some(Four)));
    assert!(redoku.place_if_valid(4, 7, Some(Two)));

    assert!(redoku.place_if_valid(5, 0, Some(Six)));
    assert!(redoku.place_if_valid(5, 1, Some(Two)));
    assert!(redoku.place_if_valid(5, 2, Some(Seven)));
    assert!(redoku.place_if_valid(5, 4, Some(Three)));
    assert!(redoku.place_if_valid(5, 6, Some(One)));
    assert!(redoku.place_if_valid(5, 7, Some(Eight)));

    assert!(redoku.place_if_valid(6, 0, Some(Seven)));
    assert!(redoku.place_if_valid(6, 1, Some(Six)));
    assert!(redoku.place_if_valid(6, 4, Some(Five)));

    assert!(redoku.place_if_valid(7, 1, Some(Eight)));
    assert!(redoku.place_if_valid(7, 7, Some(Five)));

    assert!(redoku.place_if_valid(8, 2, Some(Four)));
    assert!(redoku.place_if_valid(8, 5, Some(Six)));
    assert!(redoku.place_if_valid(8, 8, Some(Eight)));

    assert!(redoku.empty_cells() == 43);
    b.iter(|| {
        let mut cloned = redoku.clone();

        assert!(try_lone_ranger(&mut cloned));
        assert!(cloned.empty_cells() == 35);

        assert!(try_lone_ranger(&mut cloned));
        assert!(cloned.empty_cells() == 28);

        assert!(try_lone_ranger(&mut cloned));
        assert!(cloned.empty_cells() == 23);
        // Could go one more for 22..
    });
}

#[test]
fn test_twins_triplets() {
    let mut redoku = Redoku::new();

    assert!(redoku.place_if_valid(0, 0, Some(Six)));
    assert!(redoku.place_if_valid(0, 2, Some(Nine)));
    assert!(redoku.place_if_valid(0, 3, Some(One)));

    assert!(redoku.place_if_valid(1, 1, Some(Eight)));
    assert!(redoku.place_if_valid(1, 2, Some(Three)));

    assert!(redoku.place_if_valid(2, 0, Some(Seven)));
    assert!(redoku.place_if_valid(2, 2, Some(One)));
    assert!(redoku.place_if_valid(2, 3, Some(Six)));

    assert!(redoku.place_if_valid(3, 1, Some(Seven)));
    assert!(redoku.place_if_valid(3, 2, Some(Six)));
    assert!(redoku.place_if_valid(3, 3, Some(Five)));

    assert!(redoku.place_if_valid(4, 2, Some(Four)));

    assert!(redoku.place_if_valid(5, 0, Some(One)));
    assert!(redoku.place_if_valid(5, 2, Some(Five)));
    assert!(redoku.place_if_valid(5, 3, Some(Seven)));

    assert!(redoku.place_if_valid(6, 1, Some(One)));
    assert!(redoku.place_if_valid(6, 2, Some(Two)));
    assert!(redoku.place_if_valid(6, 3, Some(Eight)));

    assert!(redoku.place_if_valid(7, 1, Some(Six)));
    assert!(redoku.place_if_valid(7, 2, Some(Eight)));
    assert!(redoku.place_if_valid(7, 3, Some(Nine)));

    assert!(redoku.place_if_valid(8, 1, Some(Nine)));
    assert!(redoku.place_if_valid(8, 2, Some(Seven)));

    assert!(redoku.calculate_possible_values(0, 1) == ValueSet::new(0b0_0001_1010));
    assert!(redoku.calculate_possible_values(1, 0) == ValueSet::new(0b0_0001_1010));
    assert!(redoku.calculate_possible_values(2, 1) == ValueSet::new(0b0_0001_1010));

    try_look_for_twins_triplets(&mut redoku);

    println!("{:?}", redoku.calculate_possible_values(0, 1));
    println!("{:?}", redoku.temp_grid_values);
    assert!(redoku.calculate_possible_values(0, 1) == ValueSet::new(0b0_0001_1000));
    assert!(redoku.calculate_possible_values(2, 1) == ValueSet::new(0b0_0001_1000));

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
