use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Index, IndexMut};
use value::{CellValue, CellValueSet};

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Grid {
    Column(usize),
    Row(usize),
    Block(usize),
}

impl Index<Grid> for [CellValueSet] {
    type Output = CellValueSet;

    fn index(&self, index: Grid) -> &CellValueSet {
        match index {
            Grid::Column(val) if val < 9 => &self[val],
            Grid::Row(val) if val < 9 => &self[val + 9],
            Grid::Block(val) if val < 9 => &self[val + 18],
            _ => panic!("Block, Column, and Row must have a value between 0..9")
        }
    }
}

impl IndexMut<Grid> for [CellValueSet] {
    fn index_mut(&mut self, index: Grid) -> &mut CellValueSet {
        match index {
            Grid::Column(val) if val < 9 => &mut self[val],
            Grid::Row(val) if val < 9 => &mut self[val + 9usize],
            Grid::Block(val) if val < 9 => &mut self[val + 18usize],
            _ => panic!("Block, Column, and Row must have a value between 0..9")
        }
    }
}

// Using a Vec because a fixed size array doesn't impl Copy for BTreeSet :(
pub struct Redoku {
    cells: [Option<CellValue>; 81],
    grid_values: [CellValueSet; 27],
}

impl Redoku {
    pub fn new() -> Redoku {
        Redoku {
            cells: [None; 81],
            grid_values: [CellValueSet::new(0); 27],
        }
    }

    pub fn can_place(&self, x: usize, y: usize, value: CellValue) -> bool {
        if self.grid_values[Grid::Column(x)].contains(&value) {
            return false;
        }

        if self.grid_values[Grid::Row(y)].contains(&value) {
            return false;
        }

        let (block_x, block_y) = (x / 3, y / 3);

        if self.grid_values[Grid::Block(3 * block_y + block_x)].contains(&value) {
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

                self.grid_values[Grid::Column(x)].insert(val);
                self.grid_values[Grid::Row(y)].insert(val);
                self.grid_values[Grid::Block(3 * block_y + block_x)].insert(val);

                self.cells[9 * y + x] = Some(val);

                true
            },
            None => {
                if let Some(val) = original_value {
                    self.grid_values[Grid::Column(x)].remove(&val);
                    self.grid_values[Grid::Row(y)].remove(&val);
                    self.grid_values[Grid::Block(3 * block_y + block_x)].remove(&val);

                    self.cells[9 * y + x] = None;
                }

                true
            }
        }
    }

    pub fn empty_cells(&self) -> u8 {
        let mut cells = 81;

        for i in 0..9 {
            cells -= self.grid_values[Grid::Block(i)].len();
        }

        cells
    }

    pub fn clear(&mut self) {
        for x in 0..9 {
            for y in 0..9 {
                self.cells[9 * y + x] = None;
            }

            self.grid_values[Grid::Column(x)].clear();
            self.grid_values[Grid::Row(x)].clear();
            self.grid_values[Grid::Block(x)].clear();
        }
    }

    pub fn row_values(&self, row: usize) -> &CellValueSet {
        if row > 8 {
            panic!("No such row {} to get values for.", row);
        }

        &self.grid_values[Grid::Row(row)]
    }

    pub fn column_values(&self, column: usize) -> &CellValueSet {
        if column > 8 {
            panic!("No such column {} to get values for.", column);
        }

        &self.grid_values[Grid::Column(column)]
    }

    pub fn block_values(&self, block: usize) -> &CellValueSet {
        if block > 8 {
            panic!("No such block {} to get values for.", block);
        }

        &self.grid_values[Grid::Block(block)]
    }

    pub fn calculate_impossible_values(&self, x: usize, y: usize) -> CellValueSet {
        let (block_x, block_y) = (x / 3, y / 3);

        *self.column_values(x) | *self.row_values(y) | *self.block_values(3 * block_y + block_x)
    }

    pub fn calculate_possible_values(&self, x: usize, y: usize) -> CellValueSet {
        let values = CellValueSet::new(0b1_1111_1111);

        values - self.calculate_impossible_values(x, y)
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
    use value::CellValue::*;

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
