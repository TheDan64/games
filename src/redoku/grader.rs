use redoku::Redoku;
use std::cmp::{max, min};

#[cfg(test)]
use utils;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    Evil,
}

// Difficulty | Enum. search times | Score
// Very Easy  |      100 <         |   1
// Easy       |      100 -    999  |   2
// Medium     |    1,000 -  9,999  |   3
// Hard       |   10,000 - 99,999  |   4
// Evil       |  100,000 >         |   5

pub trait RedokuGrader {
    fn score_cell_total_count(&self) -> f32;
    fn score_cell_row_column_count(&self) -> f32;
    fn score_human_solving_techniques(&self) -> f32;
    fn grade_difficulty(&self) -> Difficulty;
}

impl RedokuGrader for Redoku {
    fn score_cell_total_count(&self) -> f32 {
        // Difficulty | Givens  | Scores
        // Very Easy  |    > 50 |   1
        // Easy       | 36 - 49 |   2
        // Medium     | 32 - 35 |   3
        // Hard       | 28 - 31 |   4
        // Evil       | 22 - 27 |   5

        match 81 - self.empty_cells() {
            givens if givens >= 50 => 1.0,
            36...50 => 2.0,
            32...36 => 3.0,
            28...32 => 4.0,
            22...28 => 5.0,
            _ => panic!("No evaluation metric for number of givens under 22"),
        }
    }

    fn score_cell_row_column_count(&self) -> f32 {
        // Difficulty | Lower bound of    |
        //            | givens in row/col | Scores
        // Very Easy  |        5          |   1
        // Easy       |        4          |   2
        // Medium     |        3          |   3
        // Hard       |        2          |   4
        // Evil       |        0          |   5

        let mut min_len = 9;

        for i in 0..9 {
            min_len = min(min_len, self.row_values(&i).len());

            if min_len == 0 {
                break;
            }

            min_len = min(min_len, self.column_values(&i).len());

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
            _ => 1.0, // REVIEW: Is this correct? Possible in a valid unstarted Redoku?
        }
    }

    fn score_human_solving_techniques(&self) -> f32 {
        // Technique                               | Score
        // Row, Column, and Block Elimination      |   1
        // Lone rangers in Block/Column/Row        |   2
        // Twins in Block/Column/Row               |   3
        // Triplets in Block/Column/Row            |   4
        // Brute-force Elimination                 |   5

        let mut max_score = 0.0;

        // Try one
        // max_score = max(max_score, ret_score)



        max_score
    }

    fn grade_difficulty(&self) -> Difficulty {
        let mut total_score = 0.4 * self.score_cell_total_count();

        total_score += 0.2 * self.score_cell_row_column_count();
        total_score += 0.2 * self.score_human_solving_techniques();

        // Enumerating Search

        match total_score.round() {
            1.0 => Difficulty::VeryEasy,
            2.0 => Difficulty::Easy,
            3.0 => Difficulty::Medium,
            4.0 => Difficulty::Hard,
            5.0 => Difficulty::Evil,
            _ => unreachable!("Scoring metric failure"),
        }
    }
}

#[test]
fn test_grade_very_easy_redoku() {
    let redoku = utils::get_very_easy_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::VeryEasy);
}

#[test]
fn test_grade_easy_redoku() {
    let redoku = utils::get_easy_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Easy);
}

#[test]
fn test_grade_medium_redoku() {
    let redoku = utils::get_medium_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Medium);
}

#[test]
fn test_grade_hard_redoku() {
    let redoku = utils::get_hard_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Hard);
}

#[test]
fn test_grade_evil_redoku() {
    let redoku = utils::get_evil_redoku();

    assert!(redoku.grade_difficulty() == Difficulty::Evil);
}
