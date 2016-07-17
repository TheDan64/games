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
    fn get_adjacent_edges(&self, visited: &HashSet<(usize, usize)>, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();

        let (x, y) = (x as isize, y as isize);

        for offsetx in -1..2isize {
            for offsety in -1..2isize {
                if offsetx == 0 && offsety == 0 {
                    continue;
                }

                if x + offsetx < 0 || x + offsetx > 8 {
                    continue;
                }

                if y + offsety < 0 || y + offsety > 8 {
                    continue;
                }

                println!("Possible edge: {}, {}", x + offsetx, y + offsety);

                if visited.contains(&((x + offsetx) as usize, (y + offsety) as usize)) {
                    println!("Edge is visited!");
                    continue;
                }

                vec.push(((x + offsetx) as usize, (y + offsety) as usize));
            }
        }

        vec
    }

    fn depth_first_search(&self, redoku: &mut Redoku, visited: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> Solution {
        use self::Solution::*;

        visited.insert((x, y));

        let no_cell_value = redoku[(x, y)] == None;

        let iterations = if no_cell_value {
            9
        } else {
            1
        };

        let mut solution = None;

        for i in 0..iterations {
            if no_cell_value {
                redoku[(x, y)] = Some(CellValue::from_usize(i + 1));

                println!("Trying to place {:?} at {}, {}", Some(CellValue::from_usize(i + 1)), x, y);

                if !redoku.is_valid_cell(x, y) {
                    redoku[(x, y)] = None;

                    println!("Not valid! Resetting ^");

                    continue;
                }
            }

            println!("From position {}, {}", x, y);

            // Very edgey code
            for (edgex, edgey) in self.get_adjacent_edges(&visited, x, y) {
                println!("Checking edge: {}, {}", edgex, edgey);

                solution = match self.depth_first_search(redoku, visited, edgex, edgey) {
                    NonUnique => return NonUnique,
                    Unique(sol) => {
                        match solution {
                            Some(_) => return NonUnique,
                            None => Some(Unique(sol)),
                        }
                    },
                    Incomplete(sol) => Some(Incomplete(sol)),
                };

                println!("Found solution: {:?}", solution);
            }

            if no_cell_value {
                redoku[(x, y)] = None
            }
        }

        println!("{:?}", solution);

        // Maybe a less complex way to check if there are empty slots?
        if redoku.empty_cells() == 0 {
            panic!("Found unique solution!"); // Debug
            return Unique(*redoku);
        }

        if let Some(sol) = solution {
            return sol;
        }

        Incomplete(*redoku)
    }

    pub fn find_unique_solution(&self, redoku: &Redoku) -> Option<Redoku> {
        use self::Solution::*;

        let mut visited = HashSet::with_capacity(81);

        match self.depth_first_search(&mut redoku.clone(), &mut visited, 0, 0) {
            Incomplete(_) => unreachable!("Logic error"),
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
fn test_easy_redoku() {
    let mut redoku = Redoku::new();

    // redoku[(, )] = Some();
    // TODO
}

#[test]
fn test_medium_redoku() {
    let mut redoku = Redoku::new();

    // TODO
}

#[test]
fn test_hard_redoku() {
    let mut redoku = Redoku::new();

    // TODO
}

#[test]
fn test_evil_redoku() {
    let mut redoku = Redoku::new();

    // TODO
}

