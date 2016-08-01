use std::cmp::PartialEq;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::ops::Index;

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

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Grid {
    Column(usize),
    Row(usize),
    Block(usize),
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

pub struct Redoku {
    cells: [Option<CellValue>; 81],
    grid_values: BTreeMap<Grid, BTreeSet<CellValue>>,
}

impl Redoku {
    pub fn new() -> Redoku {
        let mut grid_values = BTreeMap::new();

        for i in 0..9 {
            grid_values.insert(Grid::Column(i), BTreeSet::new());
            grid_values.insert(Grid::Row(i), BTreeSet::new());
            grid_values.insert(Grid::Block(i), BTreeSet::new());
        }


        Redoku {
            cells: [None; 81],
            grid_values: grid_values,
        }
    }

    pub fn can_place(&self, x: usize, y: usize, value: CellValue) -> bool {
        if self.grid_values.get(&Grid::Column(x)).unwrap().contains(&value) {
            return false;
        }

        if self.grid_values.get(&Grid::Row(y)).unwrap().contains(&value) {
            return false;
        }

        let (block_x, block_y) = (x / 3, y / 3);

        if self.grid_values.get(&Grid::Block(3 * block_y + block_x)).unwrap().contains(&value) {
            return false;
        }

        true
    }

    pub fn place_if_valid(&mut self, x: usize, y: usize, value: Option<CellValue>) -> bool {
        let original_value = self[(x, y)];

        let (block_x, block_y) = (x / 3, y / 3);

        match value {
            Some(val) => {
                if !self.can_place(x, y, val) {
                    return false;
                }

                self.grid_values.get_mut(&Grid::Column(x)).unwrap().insert(val);
                self.grid_values.get_mut(&Grid::Row(y)).unwrap().insert(val);
                self.grid_values.get_mut(&Grid::Block(3 * block_y + block_x)).unwrap().insert(val);

                self.cells[9 * y + x] = Some(val);

                true
            },
            None => {
                if let Some(val) = original_value {
                    self.grid_values.get_mut(&Grid::Column(x)).unwrap().remove(&val);
                    self.grid_values.get_mut(&Grid::Row(y)).unwrap().remove(&val);
                    self.grid_values.get_mut(&Grid::Block(3 * block_y + block_x)).unwrap().remove(&val);

                    self.cells[9 * y + x] = None;
                }

                true
            }
        }
    }

    pub fn empty_cells(&self) -> usize {
        let mut cells = 81;

        for i in 0..9 {
            cells -= self.grid_values.get(&Grid::Block(i)).unwrap().len();
        }

        cells
    }

    pub fn row_values(&self, row: usize) -> &BTreeSet<CellValue> {
        if row > 8 {
            panic!("No such row {} to get values for.", row);
        }

        self.grid_values.get(&Grid::Row(row)).unwrap()
    }

    pub fn column_values(&self, column: usize) -> &BTreeSet<CellValue> {
        if column > 8 {
            panic!("No such column {} to get values for.", column);
        }

        self.grid_values.get(&Grid::Column(column)).unwrap()
    }

    pub fn block_values(&self, block: usize) -> &BTreeSet<CellValue> {
        if block > 8 {
            panic!("No such block {} to get values for.", block);
        }

        self.grid_values.get(&Grid::Block(block)).unwrap()
    }

    pub fn calculate_impossible_values(&self, x: usize, y: usize) -> BTreeSet<CellValue> {
        let (block_x, block_y) = (x / 3, y / 3);

        &(self.column_values(x) | self.row_values(y)) | self.block_values(3 * block_y + block_x)
    }

    pub fn calculate_possible_values(&self, x: usize, y: usize) -> BTreeSet<CellValue> {
        let mut values = BTreeSet::new();
        values.insert(CellValue::One);
        values.insert(CellValue::Two);
        values.insert(CellValue::Three);
        values.insert(CellValue::Four);
        values.insert(CellValue::Five);
        values.insert(CellValue::Six);
        values.insert(CellValue::Seven);
        values.insert(CellValue::Eight);
        values.insert(CellValue::Nine);

        &values - &self.calculate_impossible_values(x, y)
    }
}

// Clone and PartialEq need to be manually implemented because
// [T; n] has issues for n > 32
impl Clone for Redoku {
    fn clone(&self) -> Redoku {
        Redoku {
            cells: self.cells,
            grid_values: self.grid_values.clone(),
        }
    }
}

impl PartialEq for Redoku {
    fn eq(&self, other: &Redoku) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if self.cells[9 * y + x] != other.cells[9 * y + x] {
                    return false;
                }
            }
        }

        self.grid_values == other.grid_values
    }

    fn ne(&self, other: &Redoku) -> bool {
        !self.eq(other)
    }
}

impl Index<(usize, usize)> for Redoku {
    type Output = Option<CellValue>;

    fn index(&self, index: (usize, usize)) -> &Option<CellValue> {
        let (x, y) = index;

        &self.cells[y * 9 + x]
    }
}

impl fmt::Debug for Redoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::char;

        let mut string = String::new();

        string.push_str("┏━━━━━━━━━━━━━━━━━━━━━━━┓");

        for y in 0..9 {
            string.push_str("\n┃");

            for x in 0..9 {
                string.push_str("│");

                if x == 3 || x == 6 {
                    string.push_str(" │");
                }

                string.push_str(&format!("{}", match self[(x, y)] {
                    Some(val) => char::from_digit(val as u32, 10).unwrap(),
                    None => '?',
                }));
            }

            string.push_str("│┃");

            if y == 2 || y == 5 {
                string.push_str("\n┃                       ┃");
            }
        }

        string.push_str("\n┗━━━━━━━━━━━━━━━━━━━━━━━┛");

        write!(f, "{}", string)
    }
}

#[test]
fn test_indexing() {
    let mut redoku = Redoku::new();

    for x in 0..9 {
        for y in 0..9 {
            assert!(redoku[(x, y)] == None);

            redoku.cells[9 * y + x] = Some(CellValue::from_usize(y + 1));

            assert!(redoku[(x, y)] == Some(CellValue::from_usize(y + 1)));
        }
    }
}

#[test]
fn test_place_if_valid() {
    use self::CellValue::*;

    let mut redoku = Redoku::new();

    // Test column
    assert!(redoku.place_if_valid(1, 1, Some(One)));

    assert!(!redoku.place_if_valid(8, 1, Some(One)));

    // Test row
    assert!(!redoku.place_if_valid(1, 8, Some(One)));

    // Test block
    assert!(redoku.place_if_valid(0, 7, Some(One)));

    assert!(!redoku.place_if_valid(2, 8, Some(One)));

    assert!(redoku.place_if_valid(1, 1, None));
}
