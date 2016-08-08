use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::iter::Iterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    One   = 0,
    Two   = 1,
    Three = 2,
    Four  = 3,
    Five  = 4,
    Six   = 5,
    Seven = 6,
    Eight = 7,
    Nine  = 8,
}

impl Value {
    pub fn from_u8(val: u8) -> Value {
        match val {
            0 => Value::One,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Eight,
            8 => Value::Nine,
            _ => panic!("Value {} is not a valid Value", val)
        }
    }
}

/// Represents a fully ordered bit set of Values where the lowest bit is Value::One
#[derive(Clone, Copy, PartialEq)]
pub struct ValueSet {
    set: u16,
}

impl ValueSet {
    pub fn new(value: u16) -> ValueSet {
        // Alternatively, this could take a &[Value]

        if value > 0b1_1111_1111 {
            panic!("Invalid starting state for ValueSet");
        }

        ValueSet {
            set: value,
        }
    }

    pub fn contains(&self, value: &Value) -> bool {
        let bit_val = 1 << *value as u16;

        self.set & bit_val == bit_val
    }

    pub fn insert(&mut self, value: Value) {
        let bit_val = 1 << value as u16;

        self.set |= bit_val;
    }

    pub fn remove(&mut self, value: &Value) {
        let bit_val = 1 << *value as u16;

        self.set &= !bit_val;
    }

    pub fn clear(&mut self) {
        self.set = 0;
    }

    pub fn len(&self) -> u8 {
        self.set.count_ones() as u8
    }

    pub fn is_empty(&self) -> bool {
        self.set == 0
    }
}

impl BitAnd for ValueSet {
    type Output = ValueSet;

    fn bitand(self, rhs: ValueSet) -> ValueSet {
        ValueSet::new(self.set & rhs.set)
    }
}

impl BitAndAssign for ValueSet {
    fn bitand_assign(&mut self, rhs: ValueSet) {
        self.set &= rhs.set
    }
}

impl BitOr for ValueSet {
    type Output = ValueSet;

    fn bitor(self, rhs: ValueSet) -> ValueSet {
        ValueSet::new(self.set | rhs.set)
    }
}

impl BitOrAssign for ValueSet {
    fn bitor_assign(&mut self, rhs: ValueSet) {
        self.set |= rhs.set
    }
}

impl Sub for ValueSet {
    type Output = ValueSet;

    fn sub(self, rhs: ValueSet) -> ValueSet {
        ValueSet::new(self.set & !rhs.set)
    }
}

impl SubAssign for ValueSet {
    fn sub_assign(&mut self, rhs: ValueSet) {
        self.set &= !rhs.set
    }
}

impl PartialOrd for ValueSet {
    fn partial_cmp(&self, other: &ValueSet) -> Option<Ordering> {
        if self.set == other.set {
            return Some(Ordering::Equal);
        }

        if (self.set & other.set) == other.set {
            return Some(Ordering::Greater);
        }

        if (self.set & other.set) == self.set {
            return Some(Ordering::Less);
        }

        None
    }
}

impl Iterator for ValueSet {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        if self.set == 0 {
            return None;
        }

        let rightmost_one = self.set & (!self.set + 1);

        self.set -= rightmost_one;

        Some(Value::from_u8((rightmost_one - 1).count_ones() as u8))
    }
}

impl fmt::Debug for ValueSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueSet(0b{:b})", self.set)
    }
}

#[test]
fn test_cell_value_set_ops() {
    use self::Value::*;

    let mut set = ValueSet::new(0);

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

    set.set = 0b1_1010_0001;

    assert!(set <= ValueSet::new(0b1_1010_0001));
    assert!(set < ValueSet::new(0b1_1110_0101));
    assert!(set >= ValueSet::new(0b1_1010_0001));
    assert!(set > ValueSet::new(0b1_0010_0001));
    assert!(!(set < ValueSet::new(0b1_0110_0001)));
    assert!(!(set > ValueSet::new(0b1_0110_0001)));
}

#[test]
fn test_cell_value_set_bitops() {
    let set = ValueSet::new(0b1_1010_0001);

    let set2 = set & ValueSet::new(0b1_1111_0000);

    assert!(set2.set == 0b1_1010_0000);

    let set3 = set | ValueSet::new(0b0_0101_1001);

    assert!(set3.set == 0b1_1111_1001);

    let set4 = set3 - set2;

    assert!(set4.set == 0b0_0101_1001);

    let mut set = ValueSet::new(0b1_1100_0011);

    set |= ValueSet::new(0b0_0110_1001);

    assert!(set.set == 0b1_1110_1011);

    set &= ValueSet::new(0b1_1000_1000);

    assert!(set.set == 0b1_1000_1000);

    set -= ValueSet::new(0b0_1111_1111);

    assert!(set.set == 0b1_0000_0000);
}
