use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Index, IndexMut};
use value::{Value, ValueSet};

#[derive(Clone, Copy, Debug)]
pub enum Grid {
    Column(u8),
    Row(u8),
    Block(u8),
}

impl Index<Grid> for [ValueSet] {
    type Output = ValueSet;

    fn index(&self, index: Grid) -> &ValueSet {
        match index {
            Grid::Column(val) if val < 9 => &self[val as usize],
            Grid::Row(val) if val < 9 => &self[val as usize + 9],
            Grid::Block(val) if val < 9 => &self[val as usize + 18],
            _ => panic!("Block, Column, and Row must have a value between 0..9")
        }
    }
}

impl IndexMut<Grid> for [ValueSet] {
    fn index_mut(&mut self, index: Grid) -> &mut ValueSet {
        match index {
            Grid::Column(val) if val < 9 => &mut self[val as usize],
            Grid::Row(val) if val < 9 => &mut self[val as usize + 9],
            Grid::Block(val) if val < 9 => &mut self[val as usize + 18],
            _ => panic!("Block, Column, and Row must have a value between 0..9")
        }
    }
}

// Using a Vec because a fixed size array doesn't impl Copy for BTreeSet :(
pub struct Redoku {
    cells: [Option<Value>; 81],
    grid_values: [ValueSet; 27],
    pub temp_grid_values: Vec<(Grid, ValueSet)>, // TODO: Remove pub (debug)
}

impl Redoku {
    pub fn new() -> Redoku {
        Redoku {
            cells: [None; 81],
            grid_values: [ValueSet::new(0); 27],
            temp_grid_values: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Redoku {
        Redoku {
            cells: [None; 81],
            grid_values: [ValueSet::new(0); 27],
            temp_grid_values: Vec::with_capacity(capacity),
        }
    }

    pub fn can_place(&self, x: u8, y: u8, value: Value) -> bool {
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

    pub fn place_if_valid(&mut self, x: u8, y: u8, value: Option<Value>) -> bool {
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

                self.cells[9 * y as usize + x as usize] = Some(val);

                true
            },
            None => {
                if let Some(val) = original_value {
                    self.grid_values[Grid::Column(x)].remove(&val);
                    self.grid_values[Grid::Row(y)].remove(&val);
                    self.grid_values[Grid::Block(3 * block_y + block_x)].remove(&val);

                    self.cells[9 * y as usize + x as usize] = None;
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
                self.cells[9 * y as usize + x as usize] = None;
            }

            self.grid_values[Grid::Column(x)].clear();
            self.grid_values[Grid::Row(x)].clear();
            self.grid_values[Grid::Block(x)].clear();
        }
    }

    pub fn row_values(&self, row: u8) -> &ValueSet {
        if row > 8 {
            panic!("No such row {} to get values for.", row);
        }

        &self.grid_values[Grid::Row(row)]
    }

    pub fn column_values(&self, column: u8) -> &ValueSet {
        if column > 8 {
            panic!("No such column {} to get values for.", column);
        }

        &self.grid_values[Grid::Column(column)]
    }

    pub fn block_values(&self, block: u8) -> &ValueSet {
        if block > 8 {
            panic!("No such block {} to get values for.", block);
        }

        &self.grid_values[Grid::Block(block)]
    }

    pub fn calculate_impossible_values(&self, x: u8, y: u8) -> ValueSet {
        let (block_x, block_y) = (x / 3, y / 3);

        *self.column_values(x) | *self.row_values(y) | *self.block_values(3 * block_y + block_x)
    }

    pub fn calculate_possible_values(&self, x: u8, y: u8) -> ValueSet {
        ValueSet::new(0b1_1111_1111) - self.calculate_impossible_values(x, y)
    }

    pub fn insert_temporary_values(&mut self, grid: Grid, values: ValueSet) {
        self.temp_grid_values.push((grid, values));

        println!("Inserted temp vals at {:?}: {:?}", grid, values);

        self.grid_values[grid] |= values;
    }

    pub fn remove_temporary_values(&mut self) {
        while let Some((grid, values)) = self.temp_grid_values.pop() {
            self.grid_values[grid] -= values;
        }
    }

    pub fn temporary_values(&self) -> usize {
        self.temp_grid_values.len()
    }
}

// Clone and PartialEq need to be manually implemented because
// [T; n] has issues for n > 32
impl Clone for Redoku {
    fn clone(&self) -> Redoku {
        Redoku {
            cells: self.cells,
            grid_values: self.grid_values,
            temp_grid_values: self.temp_grid_values.clone(),
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

        self.grid_values == other.grid_values //&& self.temp_grid_values == other.temp_grid_values
    }

    fn ne(&self, other: &Redoku) -> bool {
        !self.eq(other)
    }
}

impl Index<(u8, u8)> for Redoku {
    type Output = Option<Value>;

    fn index(&self, index: (u8, u8)) -> &Option<Value> {
        let (x, y) = index;

        &self.cells[y as usize * 9 + x as usize]
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
                    Some(val) => char::from_digit(val as u32 + 1, 10).unwrap(),
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

            redoku.cells[9 * y as usize + x as usize] = Some(Value::from_u8(y));

            assert!(redoku[(x, y)] == Some(Value::from_u8(y)));
        }
    }
}

#[test]
fn test_place_if_valid() {
    use value::Value::*;

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
