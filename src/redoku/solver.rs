#[cfg(test)]
use test::Bencher;
#[cfg(test)]
use value::Value::*;
#[cfg(test)]
use utils;

use std::marker::Sized;
use extra::rand::Randomizer;
use redoku::Redoku;

#[derive(Debug)]
enum Solution {
    Complete(Redoku, u32),
    Incomplete(u32),
    NonUnique,
}

fn depth_first_search(redoku: &mut Redoku, x: u8, y: u8, unique: bool) -> Solution {
    use self::Solution::*;

    let mut iteration_counter = Some(0);

    let (loop_iterations, no_starting_value) = if redoku[(x, y)] == None {
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

    for i in 0..loop_iterations {
        if no_starting_value && !redoku.place_if_valid(x, y, Some(i.into())) {
            continue;
        }

        if redoku.is_completed() {
            return Complete(redoku.clone(), iteration_counter.unwrap_or(0));
        }

        if let Some(ref mut count) = iteration_counter {
            *count += 1;
        }

        match depth_first_search(redoku, nextx, nexty, unique) {
            NonUnique => return NonUnique,
            Complete(redoku, iter_count) => {
                if !unique {
                    return Complete(redoku, iter_count);
                }

                match solution {
                    Some(Complete(_, _)) => return NonUnique,
                    None => solution = Some(Complete(redoku, iteration_counter.unwrap_or(0) + iter_count)),
                    _ => unreachable!("Logic error: Solution set to non unique or incomplete")
                }

                iteration_counter = None;
            },
            Incomplete(iter_count) => if let Some(ref mut count) = iteration_counter {
                *count += iter_count;
            }
        };

        if no_starting_value {
            redoku.place_if_valid(x, y, None);
        }
    }

    match solution {
        Some(Complete(redoku, iter_count)) => Complete(redoku, iter_count),
        Some(NonUnique) => unreachable!("Logic error: NonUnique set as solution"),
        Some(Incomplete(_)) => unreachable!("Logic error: Incomplete set as solution"),
        None => Incomplete(iteration_counter.unwrap_or(0)),
    }
}

pub trait RedokuSolver {
    fn find_solution(&self, unique: bool) -> Option<(Self, u32)> where Self: Sized;
    fn has_solution(&self, unique: bool) -> bool;
}

impl RedokuSolver for Redoku {
    fn find_solution(&self, unique: bool) -> Option<(Redoku, u32)> {
        use self::Solution::*;

        let mut redoku = self.clone();

        // println!("\n{:?}", redoku);

        match depth_first_search(&mut redoku, 0, 0, unique) {
            Incomplete(_) => unreachable!("Logic error: Incomplete at top level"),
            Complete(redoku, iterations) => Some((redoku, iterations)),
            NonUnique => None,
        }
    }

    fn has_solution(&self, unique: bool) -> bool {
        match self.find_solution(unique) {
            Some(_) => true,
            None => false,
        }
    }
}

#[test]
fn test_previously_unsolvable() {
    let redoku = redoku![
        ?,?,?, ?,?,?, ?,9,5,
        3,4,?, ?,?,?, ?,?,?,
        ?,?,?, ?,?,?, 1,4,3,

        9,1,3, 4,6,8, 5,7,2,
        2,5,4, 7,1,3, 6,8,9,
        8,6,7, 9,2,5, 4,3,1,

        1,7,2, 6,9,4, 3,5,8,
        4,3,8, 5,7,1, 9,2,6,
        5,9,6, 8,3,2, 7,1,4,
    ];

    assert!(redoku.has_solution(true));
}

#[test]
fn test_no_unique_solution() {
    let mut redoku = Redoku::new();

    assert!(redoku.place_if_valid(0, 0, Some(One)));

    assert!(redoku.place_if_valid(1, 1, Some(Seven)));

    assert!(redoku.place_if_valid(7, 6, Some(Three)));

    assert!(!redoku.has_solution(true));
}

#[bench]
fn test_solves_very_easy_redoku(b: &mut Bencher) {
    let redoku = utils::get_very_easy_redoku();

    b.iter(|| { assert!(redoku.has_solution(true)) })
}

#[bench]
fn test_solves_easy_redoku(b: &mut Bencher) {
    let redoku = utils::get_easy_redoku();

    b.iter(|| { assert!(redoku.has_solution(true)) })
}

#[bench]
fn test_solves_medium_redoku(b: &mut Bencher) {
    let redoku = utils::get_medium_redoku();

    b.iter(|| { assert!(redoku.has_solution(true)) })
}

#[bench]
fn test_solves_hard_redoku(b: &mut Bencher) {
    let redoku = utils::get_hard_redoku();

    b.iter(|| { assert!(redoku.has_solution(true)) })
}

#[bench]
fn test_solves_evil_redoku(b: &mut Bencher) {
    let redoku = utils::get_evil_redoku();

    b.iter(|| { assert!(redoku.has_solution(true)) })
}

#[bench]
fn test_solves_evil_redoku2(b: &mut Bencher) {
    let redoku = utils::get_evil_redoku2();

    b.iter(|| { assert!(redoku.has_solution(true)) })
}
