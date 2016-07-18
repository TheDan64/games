use std::collections::HashSet;

use redoku::CellValue;
use redoku::CellValue::*;
use redoku::{Redoku};

#[derive(Debug, PartialEq)]
enum Solution {
    Incomplete(Redoku),
    NonUnique,
    Unique(Redoku),
}

struct RedokuSolver {

}

impl RedokuSolver {
    fn depth_first_search(&self, redoku: &mut Redoku, x: usize, y: usize) -> Solution {
        use self::Solution::*;

        let no_cell_value = redoku[(x, y)] == None;

        let iterations = if no_cell_value {
            9
        } else {
            1
        };

        let mut solution = None;

        println!("Trying to place at {}, {}", x, y);

        let (nextx, nexty) = if x == 8 {
            (0, y + 1)
        } else {
            (x + 1, y)
        };

        for i in 0..iterations {
            if no_cell_value {
                redoku[(x, y)] = Some(CellValue::from_usize(i + 1));

                if !redoku.is_valid_cell(x, y) {
                    redoku[(x, y)] = None;

                    continue;
                }

                println!("{:?} may be valid at {}, {}!", redoku[(x, y)], x, y);
            }

            if redoku.empty_cells() == 0 {
                return Unique(*redoku);
            }

            match self.depth_first_search(redoku, nextx, nexty) {
                NonUnique => return NonUnique,
                Unique(sol) => {
                    match solution {
                        Some(Unique(_)) => {
                            panic!("Found NonUnique solution"); // DEBUG
                            return NonUnique
                        },
                        None => solution = Some(Unique(sol)),
                        _ => unreachable!("Logic error: solution set to non unique")
                    }
                },
                Incomplete(sol) => (),
            };
        }

        match solution {
            Some(Unique(sol)) => Unique(sol),
            Some(NonUnique) => unreachable!("Logic error: NonUnique at EODFS"),
            Some(Incomplete(_)) => unreachable!("Logic error: Incomplete at EODFS"),
            None => {
                println!("Incomplete with remaining cells: {}", redoku.empty_cells());
                println!("{:?}", redoku);

                Incomplete(*redoku)
            },
        }
    }

    pub fn find_unique_solution(&self, redoku: &Redoku) -> Option<Redoku> {
        use self::Solution::*;

        match self.depth_first_search(&mut redoku.clone(), 0, 0) {
            Incomplete(_) => unreachable!("Logic error: Incomplete at top level"),
            Unique(redoku) => Some(redoku),
            NonUnique => None,
        }
    }

    pub fn has_unique_solution(&self, redoku: &Redoku) -> bool {
        if let Some(_) = self.find_unique_solution(redoku) {
            return true;
        }

        false
    }
}

#[test]
fn test_solve_very_easy_redoku() {
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

    let solver = RedokuSolver {};

    assert!(solver.has_unique_solution(&redoku))
}

#[test]
fn test_solve_easy_redoku() {
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


    let solver = RedokuSolver {};

    assert!(solver.has_unique_solution(&redoku));
}

#[test]
fn test_medium_redoku() {
    let mut redoku = Redoku::new();

    // TODO
}

#[test]
fn test_solve_hard_redoku() {
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

    let solver = RedokuSolver {};

    assert!(solver.has_unique_solution(&redoku))
}

#[test]
fn test_evil_redoku() {
    let mut redoku = Redoku::new();

    // TODO
}

