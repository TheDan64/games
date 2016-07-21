#[cfg(test)]
use test::Bencher;

use redoku::CellValue;
#[cfg(test)]
use redoku::CellValue::*;
use redoku::Difficulty;
use redoku::Redoku;

#[derive(Debug, PartialEq)]
enum Solution {
    Incomplete,
    NonUnique,
    Unique(Redoku),
}

pub trait RedokuSolver<R, S> {
    fn depth_first_search(&mut self, x: usize, y: usize) -> S;
    fn find_unique_solution(&self) -> Option<R>;
    fn has_unique_solution(&self) -> bool;
}

impl RedokuSolver<Redoku, Solution> for Redoku {
    fn depth_first_search(&mut self, x: usize, y: usize) -> Solution {
        use self::Solution::*;

        let (iterations, no_starting_value) = if self[(x, y)] == None {
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
                if !self.place_if_valid(x, y, Some(CellValue::from_usize(i + 1))) {
                    continue;
                }
            }

            if self.empty_cells() == 0 {
                return Unique(self.clone());
            }

            match self.depth_first_search(nextx, nexty) {
                NonUnique => return NonUnique,
                Unique(sol) => {
                    match solution {
                        Some(Unique(_)) => return NonUnique,
                        None => solution = Some(Unique(sol)),
                        _ => unreachable!("Logic error: solution set to non unique")
                    }
                },
                Incomplete => (),
            };

            if no_starting_value {
                self.place_if_valid(x, y, None);
            }
        }


        match solution {
            Some(Unique(sol)) => Unique(sol),
            Some(NonUnique) => unreachable!("Logic error: NonUnique set as solution"),
            Some(Incomplete) => unreachable!("Logic error: Incomplete set as solution"),
            None => Incomplete,
        }
    }

    fn find_unique_solution(&self) -> Option<Redoku> {
        use self::Solution::*;

        // TODO: Ensure valid start state? Result<Option<Redoku>, ?>?

        let mut redoku = self.clone();

        match redoku.depth_first_search(0, 0) {
            Incomplete => unreachable!("Logic error: Incomplete at top level"),
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
    let mut redoku = Redoku::new(Difficulty::Easy);

    assert!(redoku.place_if_valid(0, 0, Some(One)));

    assert!(redoku.place_if_valid(1, 1, Some(Seven)));

    assert!(redoku.place_if_valid(7, 6, Some(Three)));

    assert!(!redoku.has_unique_solution());
}

#[bench]
fn test_solves_very_easy_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new(Difficulty::VeryEasy);

    assert!(redoku.place_if_valid(1, 0, Some(Six)));
    assert!(redoku.place_if_valid(2, 0, Some(Seven)));
    assert!(redoku.place_if_valid(3, 0, Some(Four)));
    assert!(redoku.place_if_valid(4, 0, Some(Two)));
    assert!(redoku.place_if_valid(5, 0, Some(Five)));
    assert!(redoku.place_if_valid(8, 0, Some(Nine)));

    assert!(redoku.place_if_valid(3, 1, Some(One)));
    assert!(redoku.place_if_valid(4, 1, Some(Eight)));
    assert!(redoku.place_if_valid(7, 1, Some(Six)));

    assert!(redoku.place_if_valid(0, 2, Some(Eight)));
    assert!(redoku.place_if_valid(1, 2, Some(Nine)));
    assert!(redoku.place_if_valid(3, 2, Some(Six)));
    assert!(redoku.place_if_valid(5, 2, Some(Seven)));
    assert!(redoku.place_if_valid(7, 2, Some(Five)));
    assert!(redoku.place_if_valid(8, 2, Some(Two)));

    assert!(redoku.place_if_valid(0, 3, Some(Four)));
    assert!(redoku.place_if_valid(4, 3, Some(Six)));
    assert!(redoku.place_if_valid(6, 3, Some(Nine)));
    assert!(redoku.place_if_valid(7, 3, Some(One)));
    assert!(redoku.place_if_valid(8, 3, Some(Three)));

    assert!(redoku.place_if_valid(0, 4, Some(Six)));
    assert!(redoku.place_if_valid(2, 4, Some(Two)));
    assert!(redoku.place_if_valid(3, 4, Some(Three)));
    assert!(redoku.place_if_valid(4, 4, Some(Nine)));
    assert!(redoku.place_if_valid(5, 4, Some(Four)));
    assert!(redoku.place_if_valid(6, 4, Some(Five)));
    assert!(redoku.place_if_valid(7, 4, Some(Seven)));

    assert!(redoku.place_if_valid(0, 5, Some(Nine)));
    assert!(redoku.place_if_valid(1, 5, Some(Seven)));
    assert!(redoku.place_if_valid(2, 5, Some(Three)));
    assert!(redoku.place_if_valid(3, 5, Some(Eight)));
    assert!(redoku.place_if_valid(5, 5, Some(One)));
    assert!(redoku.place_if_valid(6, 5, Some(Six)));
    assert!(redoku.place_if_valid(7, 5, Some(Two)));

    assert!(redoku.place_if_valid(3, 6, Some(Two)));
    assert!(redoku.place_if_valid(4, 6, Some(Four)));
    assert!(redoku.place_if_valid(5, 6, Some(Three)));
    assert!(redoku.place_if_valid(6, 6, Some(Seven)));
    assert!(redoku.place_if_valid(7, 6, Some(Nine)));
    assert!(redoku.place_if_valid(8, 6, Some(Five)));

    assert!(redoku.place_if_valid(1, 7, Some(Two)));
    assert!(redoku.place_if_valid(2, 7, Some(Four)));
    assert!(redoku.place_if_valid(3, 7, Some(Nine)));
    assert!(redoku.place_if_valid(4, 7, Some(Seven)));
    assert!(redoku.place_if_valid(5, 7, Some(Six)));
    assert!(redoku.place_if_valid(6, 7, Some(Eight)));

    assert!(redoku.place_if_valid(1, 8, Some(Three)));
    assert!(redoku.place_if_valid(3, 8, Some(Five)));
    assert!(redoku.place_if_valid(4, 8, Some(One)));
    assert!(redoku.place_if_valid(5, 8, Some(Eight)));
    assert!(redoku.place_if_valid(6, 8, Some(Two)));

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_easy_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new(Difficulty::Easy);

    redoku.place_if_valid(0, 0, Some(Two));
    redoku.place_if_valid(4, 0, Some(Five));
    redoku.place_if_valid(5, 0, Some(Seven));
    redoku.place_if_valid(6, 0, Some(Three));
    redoku.place_if_valid(7, 0, Some(Eight));
    redoku.place_if_valid(8, 0, Some(Nine));

    redoku.place_if_valid(1, 1, Some(Three));
    redoku.place_if_valid(3, 1, Some(Eight));
    redoku.place_if_valid(4, 1, Some(Nine));
    redoku.place_if_valid(5, 1, Some(One));

    redoku.place_if_valid(0, 2, Some(Seven));
    redoku.place_if_valid(2, 2, Some(Nine));
    redoku.place_if_valid(5, 2, Some(Three));
    redoku.place_if_valid(7, 2, Some(One));
    redoku.place_if_valid(8, 2, Some(Six));

    redoku.place_if_valid(1, 3, Some(Seven));
    redoku.place_if_valid(2, 3, Some(Three));
    redoku.place_if_valid(4, 3, Some(Eight));
    redoku.place_if_valid(5, 3, Some(Nine));
    redoku.place_if_valid(6, 3, Some(Two));
    redoku.place_if_valid(7, 3, Some(Six));

    redoku.place_if_valid(2, 4, Some(Two));
    redoku.place_if_valid(3, 4, Some(Five));
    redoku.place_if_valid(5, 4, Some(Six));

    redoku.place_if_valid(1, 5, Some(Nine));
    redoku.place_if_valid(3, 5, Some(Three));
    redoku.place_if_valid(5, 5, Some(Four));
    redoku.place_if_valid(7, 5, Some(Five));
    redoku.place_if_valid(8, 5, Some(Seven));

    redoku.place_if_valid(1, 6, Some(Five));
    redoku.place_if_valid(3, 6, Some(Nine));
    redoku.place_if_valid(5, 6, Some(Eight));
    redoku.place_if_valid(7, 6, Some(Two));

    redoku.place_if_valid(0, 7, Some(Nine));
    redoku.place_if_valid(1, 7, Some(Two));
    redoku.place_if_valid(2, 7, Some(Eight));
    redoku.place_if_valid(3, 7, Some(Seven));
    redoku.place_if_valid(5, 7, Some(Five));
    redoku.place_if_valid(6, 7, Some(Six));

    redoku.place_if_valid(4, 8, Some(Three));

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_medium_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new(Difficulty::Medium);

    redoku.place_if_valid(1, 0, Some(Nine));
    redoku.place_if_valid(3, 0, Some(Six));
    redoku.place_if_valid(5, 0, Some(One));

    redoku.place_if_valid(4, 1, Some(Three));
    redoku.place_if_valid(6, 1, Some(Nine));
    redoku.place_if_valid(8, 1, Some(One));

    redoku.place_if_valid(1, 2, Some(Three));
    redoku.place_if_valid(3, 2, Some(Two));
    redoku.place_if_valid(5, 2, Some(Eight));

    redoku.place_if_valid(0, 3, Some(Seven));
    redoku.place_if_valid(2, 3, Some(Nine));
    redoku.place_if_valid(8, 3, Some(Four));

    redoku.place_if_valid(1, 4, Some(Four));
    redoku.place_if_valid(3, 4, Some(Three));
    redoku.place_if_valid(5, 4, Some(Seven));
    redoku.place_if_valid(7, 4, Some(Nine));

    redoku.place_if_valid(0, 5, Some(Eight));
    redoku.place_if_valid(2, 5, Some(Three));
    redoku.place_if_valid(4, 5, Some(One));
    redoku.place_if_valid(6, 5, Some(Five));
    redoku.place_if_valid(8, 5, Some(Seven));

    redoku.place_if_valid(1, 6, Some(Five));
    redoku.place_if_valid(3, 6, Some(Seven));
    redoku.place_if_valid(5, 6, Some(Two));
    redoku.place_if_valid(7, 6, Some(One));

    redoku.place_if_valid(0, 7, Some(Nine));
    redoku.place_if_valid(2, 7, Some(Four));
    redoku.place_if_valid(4, 7, Some(Five));
    redoku.place_if_valid(6, 7, Some(Seven));
    redoku.place_if_valid(8, 7, Some(Six));

    redoku.place_if_valid(1, 8, Some(One));
    redoku.place_if_valid(3, 8, Some(Nine));
    redoku.place_if_valid(5, 8, Some(Six));
    redoku.place_if_valid(7, 8, Some(Five));
    redoku.place_if_valid(8, 8, Some(Eight));

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_hard_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new(Difficulty::Hard);

    redoku.place_if_valid(7, 0, Some(Three));
    redoku.place_if_valid(8, 0, Some(Two));

    redoku.place_if_valid(0, 1, Some(Three));
    redoku.place_if_valid(1, 1, Some(Six));

    redoku.place_if_valid(6, 2, Some(Five));
    redoku.place_if_valid(8, 2, Some(Eight));

    redoku.place_if_valid(0, 3, Some(Eight));
    redoku.place_if_valid(1, 3, Some(Seven));

    redoku.place_if_valid(1, 4, Some(Nine));
    redoku.place_if_valid(5, 4, Some(Three));
    redoku.place_if_valid(7, 4, Some(Four));

    redoku.place_if_valid(0, 5, Some(Six));
    redoku.place_if_valid(3, 5, Some(Eight));

    redoku.place_if_valid(5, 6, Some(Two));
    redoku.place_if_valid(8, 6, Some(Three));

    redoku.place_if_valid(0, 7, Some(Five));
    redoku.place_if_valid(2, 7, Some(One));
    redoku.place_if_valid(3, 7, Some(Six));
    redoku.place_if_valid(4, 7, Some(Three));
    redoku.place_if_valid(6, 7, Some(Four));

    redoku.place_if_valid(1, 8, Some(Three));
    redoku.place_if_valid(2, 8, Some(Nine));
    redoku.place_if_valid(3, 8, Some(One));
    redoku.place_if_valid(4, 8, Some(Four));
    redoku.place_if_valid(5, 8, Some(Eight));
    redoku.place_if_valid(6, 8, Some(Seven));
    redoku.place_if_valid(7, 8, Some(Five));
    redoku.place_if_valid(8, 8, Some(Six));

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_evil_redoku(b: &mut Bencher) {
    let mut redoku = Redoku::new(Difficulty::Evil);

    redoku.place_if_valid(6, 1, Some(Five));
    redoku.place_if_valid(7, 1, Some(Two));
    redoku.place_if_valid(8, 1, Some(Three));

    redoku.place_if_valid(7, 2, Some(One));
    redoku.place_if_valid(8, 2, Some(Eight));

    redoku.place_if_valid(2, 4, Some(Nine));
    redoku.place_if_valid(4, 4, Some(Seven));
    redoku.place_if_valid(5, 4, Some(Four));
    redoku.place_if_valid(7, 4, Some(Six));

    redoku.place_if_valid(2, 5, Some(Four));
    redoku.place_if_valid(3, 5, Some(Six));
    redoku.place_if_valid(4, 5, Some(One));
    redoku.place_if_valid(8, 5, Some(Seven));

    redoku.place_if_valid(1, 6, Some(Five));
    redoku.place_if_valid(2, 6, Some(Eight));
    redoku.place_if_valid(4, 6, Some(Four));
    redoku.place_if_valid(5, 6, Some(Three));

    redoku.place_if_valid(1, 7, Some(Four));
    redoku.place_if_valid(4, 7, Some(Two));
    redoku.place_if_valid(7, 7, Some(Three));

    redoku.place_if_valid(1, 8, Some(Six));
    redoku.place_if_valid(2, 8, Some(Seven));
    redoku.place_if_valid(4, 8, Some(Eight));
    redoku.place_if_valid(5, 8, Some(One));
    redoku.place_if_valid(7, 8, Some(Nine));
    redoku.place_if_valid(8, 8, Some(Four));

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

