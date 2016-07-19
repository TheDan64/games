use std::collections::HashSet;
use test::Bencher;

use redoku::CellValue;
use redoku::CellValue::*;
use redoku::{Redoku};

#[derive(Debug, PartialEq)]
enum Solution {
    Incomplete(Redoku),
    NonUnique,
    Unique(Redoku),
}

trait RedokuSolver {
    fn depth_first_search(redoku: &mut Redoku, x: usize, y: usize) -> Solution;
    fn find_unique_solution(&self) -> Option<Redoku>;
    fn has_unique_solution(&self) -> bool;
}

impl RedokuSolver for Redoku {
    fn depth_first_search(redoku: &mut Redoku, x: usize, y: usize) -> Solution {
        use self::Solution::*;

        let (iterations, no_starting_value) = if redoku[(x, y)] == None {
            (9, true)
        } else {
            (1, false)
        };

        let (nextx, nexty) = if x == 8 {
            (0, y + 1)
        } else {
            (x + 1, y)
        };

        let mut solution = None;

        for i in 0..iterations {
            if no_starting_value {
                redoku[(x, y)] = Some(CellValue::from_usize(i + 1));

                if !redoku.is_valid_cell(x, y) {
                    redoku[(x, y)] = None;

                    continue;
                }
            }

            if redoku.empty_cells() == 0 {
                return Unique(*redoku);
            }

            match Redoku::depth_first_search(redoku, nextx, nexty) {
                NonUnique => return NonUnique,
                Unique(sol) => {
                    match solution {
                        Some(Unique(_)) => return NonUnique,
                        None => solution = Some(Unique(sol)),
                        _ => unreachable!("Logic error: solution set to non unique")
                    }
                },
                Incomplete(_) => (),
            };
        }

        if no_starting_value {
            redoku[(x, y)] = None;
        }

        match solution {
            Some(Unique(sol)) => Unique(sol),
            Some(NonUnique) => unreachable!("Logic error: NonUnique set as solution"),
            Some(Incomplete(_)) => unreachable!("Logic error: Incomplete set as solution"),
            None => Incomplete(*redoku),
        }
    }

    fn find_unique_solution(&self) -> Option<Redoku> {
        use self::Solution::*;

        match Redoku::depth_first_search(&mut self.clone(), 0, 0) {
            Incomplete(_) => unreachable!("Logic error: Incomplete at top level"),
            Unique(redoku) => Some(redoku),
            NonUnique => None,
        }
    }

    fn has_unique_solution(&self) -> bool {
        match self.find_unique_solution() {
            Some(_) => true,
            None => false,
        }
    }
}

#[test]
fn test_no_unique_solution() {
    let mut redoku = Redoku::new();

    redoku[(0, 0)] = Some(One);
    // redoku[(0, 1)] = Some(One); // TODO: Solver maybe should check for invalid start state

    redoku[(1, 1)] = Some(Seven);

    redoku[(7, 6)] = Some(Three);

    assert!(!redoku.has_unique_solution());
}

#[bench]
fn test_solves_very_easy_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku[(1, 0)] = Some(Six);
    redoku[(2, 0)] = Some(Seven);
    redoku[(3, 0)] = Some(Four);
    redoku[(4, 0)] = Some(Two);
    redoku[(5, 0)] = Some(Five);
    redoku[(8, 0)] = Some(Nine);

    redoku[(3, 1)] = Some(One);
    redoku[(4, 1)] = Some(Eight);
    redoku[(7, 1)] = Some(Six);

    redoku[(0, 2)] = Some(Eight);
    redoku[(1, 2)] = Some(Nine);
    redoku[(3, 2)] = Some(Six);
    redoku[(5, 2)] = Some(Seven);
    redoku[(7, 2)] = Some(Five);
    redoku[(8, 2)] = Some(Two);

    redoku[(0, 3)] = Some(Four);
    redoku[(4, 3)] = Some(Six);
    redoku[(6, 3)] = Some(Nine);
    redoku[(7, 3)] = Some(One);
    redoku[(8, 3)] = Some(Three);

    redoku[(0, 4)] = Some(Six);
    redoku[(2, 4)] = Some(Two);
    redoku[(3, 4)] = Some(Three);
    redoku[(4, 4)] = Some(Nine);
    redoku[(5, 4)] = Some(Four);
    redoku[(6, 4)] = Some(Five);
    redoku[(7, 4)] = Some(Seven);

    redoku[(0, 5)] = Some(Nine);
    redoku[(1, 5)] = Some(Seven);
    redoku[(2, 5)] = Some(Three);
    redoku[(3, 5)] = Some(Eight);
    redoku[(5, 5)] = Some(One);
    redoku[(6, 5)] = Some(Six);
    redoku[(7, 5)] = Some(Two);

    redoku[(3, 6)] = Some(Two);
    redoku[(4, 6)] = Some(Four);
    redoku[(5, 6)] = Some(Three);
    redoku[(6, 6)] = Some(Seven);
    redoku[(7, 6)] = Some(Nine);
    redoku[(8, 6)] = Some(Five);

    redoku[(1, 7)] = Some(Two);
    redoku[(2, 7)] = Some(Four);
    redoku[(3, 7)] = Some(Nine);
    redoku[(4, 7)] = Some(Seven);
    redoku[(5, 7)] = Some(Six);
    redoku[(6, 7)] = Some(Eight);

    redoku[(1, 8)] = Some(Three);
    redoku[(3, 8)] = Some(Five);
    redoku[(4, 8)] = Some(One);
    redoku[(5, 8)] = Some(Eight);
    redoku[(6, 8)] = Some(Two);

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_easy_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku[(0, 0)] = Some(Two);
    redoku[(4, 0)] = Some(Five);
    redoku[(5, 0)] = Some(Seven);
    redoku[(6, 0)] = Some(Three);
    redoku[(7, 0)] = Some(Eight);
    redoku[(8, 0)] = Some(Nine);

    redoku[(1, 1)] = Some(Three);
    redoku[(3, 1)] = Some(Eight);
    redoku[(4, 1)] = Some(Nine);
    redoku[(5, 1)] = Some(One);

    redoku[(0, 2)] = Some(Seven);
    redoku[(2, 2)] = Some(Nine);
    redoku[(5, 2)] = Some(Three);
    redoku[(7, 2)] = Some(One);
    redoku[(8, 2)] = Some(Six);

    redoku[(1, 3)] = Some(Seven);
    redoku[(2, 3)] = Some(Three);
    redoku[(4, 3)] = Some(Eight);
    redoku[(5, 3)] = Some(Nine);
    redoku[(6, 3)] = Some(Two);
    redoku[(7, 3)] = Some(Six);

    redoku[(2, 4)] = Some(Two);
    redoku[(3, 4)] = Some(Five);
    redoku[(5, 4)] = Some(Six);

    redoku[(1, 5)] = Some(Nine);
    redoku[(3, 5)] = Some(Three);
    redoku[(5, 5)] = Some(Four);
    redoku[(7, 5)] = Some(Five);
    redoku[(8, 5)] = Some(Seven);

    redoku[(1, 6)] = Some(Five);
    redoku[(3, 6)] = Some(Nine);
    redoku[(5, 6)] = Some(Eight);
    redoku[(7, 6)] = Some(Two);

    redoku[(0, 7)] = Some(Nine);
    redoku[(1, 7)] = Some(Two);
    redoku[(2, 7)] = Some(Eight);
    redoku[(3, 7)] = Some(Seven);
    redoku[(5, 7)] = Some(Five);
    redoku[(6, 7)] = Some(Six);

    redoku[(4, 8)] = Some(Three);

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_medium_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku[(1, 0)] = Some(Nine);
    redoku[(3, 0)] = Some(Six);
    redoku[(5, 0)] = Some(One);

    redoku[(4, 1)] = Some(Three);
    redoku[(6, 1)] = Some(Nine);
    redoku[(8, 1)] = Some(One);

    redoku[(1, 2)] = Some(Three);
    redoku[(3, 2)] = Some(Two);
    redoku[(5, 2)] = Some(Eight);

    redoku[(0, 3)] = Some(Seven);
    redoku[(2, 3)] = Some(Nine);
    redoku[(8, 3)] = Some(Four);

    redoku[(1, 4)] = Some(Four);
    redoku[(3, 4)] = Some(Three);
    redoku[(5, 4)] = Some(Seven);
    redoku[(7, 4)] = Some(Nine);

    redoku[(0, 5)] = Some(Eight);
    redoku[(2, 5)] = Some(Three);
    redoku[(4, 5)] = Some(One);
    redoku[(6, 5)] = Some(Five);
    redoku[(8, 5)] = Some(Seven);

    redoku[(1, 6)] = Some(Five);
    redoku[(3, 6)] = Some(Seven);
    redoku[(5, 6)] = Some(Two);
    redoku[(7, 6)] = Some(One);

    redoku[(0, 7)] = Some(Nine);
    redoku[(2, 7)] = Some(Four);
    redoku[(4, 7)] = Some(Five);
    redoku[(6, 7)] = Some(Seven);
    redoku[(8, 7)] = Some(Six);

    redoku[(1, 8)] = Some(One);
    redoku[(3, 8)] = Some(Nine);
    redoku[(5, 8)] = Some(Six);
    redoku[(7, 8)] = Some(Five);
    redoku[(8, 8)] = Some(Eight);

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_hard_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku[(7, 0)] = Some(Three);
    redoku[(8, 0)] = Some(Two);

    redoku[(0, 1)] = Some(Three);
    redoku[(1, 1)] = Some(Six);

    redoku[(6, 2)] = Some(Five);
    redoku[(8, 2)] = Some(Eight);

    redoku[(0, 3)] = Some(Eight);
    redoku[(1, 3)] = Some(Seven);

    redoku[(1, 4)] = Some(Nine);
    redoku[(5, 4)] = Some(Three);
    redoku[(7, 4)] = Some(Four);

    redoku[(0, 5)] = Some(Six);
    redoku[(3, 5)] = Some(Eight);

    redoku[(5, 6)] = Some(Two);
    redoku[(8, 6)] = Some(Three);

    redoku[(0, 7)] = Some(Five);
    redoku[(2, 7)] = Some(One);
    redoku[(3, 7)] = Some(Six);
    redoku[(4, 7)] = Some(Three);
    redoku[(6, 7)] = Some(Four);

    redoku[(1, 8)] = Some(Three);
    redoku[(2, 8)] = Some(Nine);
    redoku[(3, 8)] = Some(One);
    redoku[(4, 8)] = Some(Four);
    redoku[(5, 8)] = Some(Eight);
    redoku[(6, 8)] = Some(Seven);
    redoku[(7, 8)] = Some(Five);
    redoku[(8, 8)] = Some(Six);

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_evil_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new();

    redoku[(6, 1)] = Some(Five);
    redoku[(7, 1)] = Some(Two);
    redoku[(8, 1)] = Some(Three);

    redoku[(7, 2)] = Some(One);
    redoku[(8, 2)] = Some(Eight);

    redoku[(2, 4)] = Some(Nine);
    redoku[(4, 4)] = Some(Seven);
    redoku[(5, 4)] = Some(Four);
    redoku[(7, 4)] = Some(Six);

    redoku[(2, 5)] = Some(Four);
    redoku[(3, 5)] = Some(Six);
    redoku[(4, 5)] = Some(One);
    redoku[(8, 5)] = Some(Seven);

    redoku[(1, 6)] = Some(Five);
    redoku[(2, 6)] = Some(Eight);
    redoku[(4, 6)] = Some(Four);
    redoku[(5, 6)] = Some(Three);

    redoku[(1, 7)] = Some(Four);
    redoku[(4, 7)] = Some(Two);
    redoku[(7, 7)] = Some(Three);

    redoku[(1, 8)] = Some(Six);
    redoku[(2, 8)] = Some(Seven);
    redoku[(4, 8)] = Some(Eight);
    redoku[(5, 8)] = Some(One);
    redoku[(7, 8)] = Some(Nine);
    redoku[(8, 8)] = Some(Four);

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

