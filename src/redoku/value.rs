use std::iter::Iterator;
use std::ops::{BitAnd, BitOr, Sub};

#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum CellValue {
    One   = 1,
    Two   = 2,
    Three = 3,
    Four  = 4,
    Five  = 5,
    Six   = 6,
    Seven = 7,
    Eight = 8,
    Nine  = 9,
}

impl CellValue {
    pub fn from_usize(val: usize) -> CellValue {
        match val {
            1 => CellValue::One,
            2 => CellValue::Two,
            3 => CellValue::Three,
            4 => CellValue::Four,
            5 => CellValue::Five,
            6 => CellValue::Six,
            7 => CellValue::Seven,
            8 => CellValue::Eight,
            9 => CellValue::Nine,
            _ => panic!("Value {} is not a valid CellValue", val)
        }
    }
}

/// Represents a fully ordered bit set of CellValues where the lowest bit is 1
#[derive(Clone, Copy, PartialEq)]
pub struct CellValueSet {
    set: u16,
}

impl CellValueSet {
    pub fn new(value: u16) -> CellValueSet {
        CellValueSet {
            set: value,
        }
    }

    pub fn contains(&self, value: &CellValue) -> bool {
        let bit_val = 1 << (*value as u16 - 1);

        self.set & bit_val == bit_val
    }

    pub fn insert(&mut self, value: CellValue) {
        let bit_val = 1 << (value as u16 - 1);

        self.set |= bit_val;
    }

    pub fn remove(&mut self, value: &CellValue) {
        let bit_val = 1 << (*value as u16 - 1);

        self.set &= !bit_val;
    }

    pub fn clear(&mut self) {
        self.set = 0;
    }

    pub fn len(&self) -> u8 {
        self.set.count_ones() as u8
    }

    pub fn iter(&mut self) -> CellValueSetIter {
        CellValueSetIter {
            set: self
        }
    }
}

impl BitAnd for CellValueSet {
    type Output = CellValueSet;

    fn bitand(self, rhs: CellValueSet) -> CellValueSet {
        CellValueSet::new(self.set & rhs.set)
    }
}

impl BitOr for CellValueSet {
    type Output = CellValueSet;

    fn bitor(self, rhs: CellValueSet) -> CellValueSet {
        CellValueSet::new(self.set | rhs.set)
    }
}

impl Sub for CellValueSet {
    type Output = CellValueSet;

    fn sub(self, rhs: CellValueSet) -> CellValueSet {
        CellValueSet::new(self.set & !rhs.set)
    }
}

pub struct CellValueSetIter<'a> {
    set: &'a mut CellValueSet,
}

impl<'a> Iterator for CellValueSetIter<'a> {
    type Item = CellValue;

    fn next(&mut self) -> Option<CellValue> {
        if self.set.set == 0 {
            return None;
        }

        let rightmost_one = self.set.set & (!self.set.set + 1);

        self.set.set -= rightmost_one;

        Some(CellValue::from_usize((rightmost_one - 1).count_ones() as usize + 1))
    }
}

#[test]
fn test_cell_value_set_ops() {
    use self::CellValue::*;

    let mut set = CellValueSet::new(0);

    set.insert(One);
    set.insert(Five);
    set.insert(Eight);

    assert!(set.set == 0b0_1001_0001);

    if set.contains(&Five) {
        set.remove(&Five);
    }

    assert!(set.set == 0b0_1000_0001);
    assert!(set.len() == 2);
    set.clear();
    assert!(set.set == 0);
}

#[test]
fn test_cell_value_set_bitops() {
    let set = CellValueSet::new(0b1_1010_0001);

    let set2 = set & CellValueSet::new(0b1_1111_0000);

    assert!(set2.set == 0b1_1010_0000);

    let set3 = set | CellValueSet::new(0b0_0101_1001);

    assert!(set3.set == 0b1_1111_1001);

    let set4 = set3 - set2;

    assert!(set4.set == 0b0_0101_1001);
}
