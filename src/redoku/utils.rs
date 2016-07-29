use extra::rand::Randomizer;
use redoku::Redoku;
use redoku::CellValue;
use redoku::CellValue::*;
use std::ops::Range;

pub fn rand_u8(rand: &mut Randomizer, range: &mut Range<u8>) -> u8 {
    let low = range.next().unwrap();
    let high = range.last().unwrap();
    let range = high - low;
    let zone = 255 - 255 % range;

    loop {
        let val = rand.read_u8();

        if val < zone {
            return low.wrapping_add(val % range);
        }
    }
}

// Should be a uniform distribution by disallowing values
// 252 - 255 due to 252 (0 - 251) dividing 9 cleanly
pub fn random_cell_value(rand: &mut Randomizer) -> CellValue {
    loop {
        let val = rand.read_u8();

        if val < 252 {
            return CellValue::from_usize((val % 9) as usize + 1)
        }
    }
}

pub fn get_very_easy_redoku() -> Redoku {
    let mut redoku = Redoku::new();

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

    redoku
}

pub fn get_easy_redoku() -> Redoku {
    let mut redoku = Redoku::new();

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

    redoku
}

pub fn get_medium_redoku() -> Redoku {
    let mut redoku = Redoku::new();

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

    redoku
}

pub fn get_hard_redoku() -> Redoku {
    let mut redoku = Redoku::new();

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

    redoku
}

pub fn get_evil_redoku() -> Redoku {
    let mut redoku = Redoku::new();

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

    redoku
}

// #[test]
// fn test_rand_u8() {
//     let rand = Randomizer::new(0);

//     for i in 0..9 {
//         let u = rand_u8(&mut rand, &mut 10..18);

//         assert!(u >= 10 && u < 18);
//     }
// }
