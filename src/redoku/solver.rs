#[cfg(test)]
use test::Bencher;
#[cfg(test)]
use value::Value::*;
// #[cfg(test)]
use utils;

use extra::rand::Randomizer;
use value::Value;
use redoku::Redoku;

#[derive(Debug, PartialEq)]
enum Solution {
    Incomplete,
    NonUnique,
    Unique,
}

fn depth_first_search(redoku: &mut Redoku, x: u8, y: u8) -> Solution {
    use self::Solution::*;

    let mut rand = Randomizer::new(0);

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
        if no_starting_value && !redoku.place_if_valid(x, y, Some(Value::from_u8(i))) {
            continue;
        }

        // println!("{:?}", redoku);

        if redoku.empty_cells() == 0 {
            return Unique;
        }

        match depth_first_search(redoku, nextx, nexty) {
            NonUnique => return NonUnique,
            Unique => match solution {
                Some(Unique) => return NonUnique,
                None => solution = Some(Unique),
                _ => unreachable!("Logic error: Solution set to non unique or imcomplete")
            },
            Incomplete => (),
        };

        if no_starting_value {
            redoku.place_if_valid(x, y, None);
        }
    }

    match solution {
        Some(Unique) => Unique,
        Some(NonUnique) => unreachable!("Logic error: NonUnique set as solution"),
        Some(Incomplete) => unreachable!("Logic error: Incomplete set as solution"),
        None => Incomplete,
    }
}

pub trait RedokuSolver<R, S> {
    fn find_unique_solution(&self) -> Option<R>;
    fn has_unique_solution(&self) -> bool;
}

impl RedokuSolver<Redoku, Solution> for Redoku {
    fn find_unique_solution(&self) -> Option<Redoku> {
        use self::Solution::*;

        let mut redoku = self.clone();

        match depth_first_search(&mut redoku, 0, 0) {
            Incomplete => unreachable!("Logic error: Incomplete at top level"),
            Unique => Some(redoku),
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

    assert!(redoku.place_if_valid(0, 0, Some(One)));

    assert!(redoku.place_if_valid(1, 1, Some(Seven)));

    assert!(redoku.place_if_valid(7, 6, Some(Three)));

    assert!(!redoku.has_unique_solution());
}

#[bench]
fn test_solves_very_easy_redoku(b: &mut Bencher) {
    let redoku = utils::get_very_easy_redoku();

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_easy_redoku(b: &mut Bencher) {
    let redoku = utils::get_easy_redoku();

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_medium_redoku(b: &mut Bencher) {
    let redoku = utils::get_medium_redoku();

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_hard_redoku(b: &mut Bencher) {
    let redoku = utils::get_hard_redoku();

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}

#[bench]
fn test_solves_evil_redoku(b: &mut Bencher) {
    let redoku = utils::get_evil_redoku();

    b.iter(|| { assert!(redoku.has_unique_solution()) })
}
