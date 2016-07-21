use std::collections::HashSet;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    Evil,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cell {
    value: Option<CellValue>,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            value: None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct CellBlock {
    cells: [Cell; 9],
}

impl CellBlock {
    fn new() -> CellBlock {
        CellBlock {
            cells: [Cell::new(); 9],
        }
    }
}

impl Index<(usize, usize)> for CellBlock {
    type Output = Option<CellValue>;

    fn index(&self, index: (usize, usize)) -> &Option<CellValue> {
        let (x, y) = index;

        if x > 2 || y > 2 {
            panic!("Index values ({}, {}) are out of bounds", x, y);
        }

        &self.cells[x + 3 * y].value
    }
}

impl IndexMut<(usize, usize)> for CellBlock {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Option<CellValue> {
        let (x, y) = index;

        if x > 2 || y > 2 {
            panic!("Index values ({}, {}) are out of bounds", x, y);
        }

        &mut self.cells[x + 3 * y].value
    }
}

// REVIEW: Maybe it makes more sense to do [CellValue; 81]? or [[CellValue; 9]; 9]?
// Cell/CellBlocks haven't been of particular need so far even
// though they seemed like a good idea initially.
#[derive(Clone, PartialEq)]
pub struct Redoku {
    difficulty: Difficulty,
    cell_blocks: [CellBlock; 9],
    row_values: HashSet<(usize, CellValue)>,
    column_values: HashSet<(usize, CellValue)>,
    block_values: HashSet<(usize, usize, CellValue)>,
}

impl Redoku {
    pub fn new(difficulty: Difficulty) -> Redoku {
        Redoku {
            difficulty: difficulty,
            cell_blocks: [CellBlock::new(); 9],
            row_values: HashSet::with_capacity(81),
            column_values: HashSet::with_capacity(81),
            block_values: HashSet::with_capacity(81),
        }
    }

    pub fn place_if_valid(&mut self, x: usize, y: usize, value: Option<CellValue>) -> bool {
        let original_value = self[(x, y)];

        match value {
            Some(val) => {
                if !self.column_values.contains(&(x, val)) && !self.row_values.contains(&(y, val)) && !self.block_values.contains(&(x / 3, y / 3, val)) {
                    self.column_values.insert((x, val));
                    self.row_values.insert((y, val));
                    self.block_values.insert((x / 3, y / 3, val));

                    self[(x, y)] = Some(val);

                    return true;
                }

                false
            },
            None => {
                if let Some(val) = original_value {
                    self.column_values.remove(&(x, val));
                    self.row_values.remove(&(y, val));
                    self.block_values.remove(&(x / 3, y / 3, val));

                    self[(x, y)] = None;
                }

                true
            }
        }
    }

    pub fn empty_cells(&self) -> usize {
        81 - self.row_values.len()
    }
}

impl Index<(usize, usize)> for Redoku {
    type Output = Option<CellValue>;

    fn index(&self, index: (usize, usize)) -> &Option<CellValue> {
        let (x, y) = index;

        match (x, y) {
            (0...2, 0...2) => &self.cell_blocks[0][(x, y)],
            (3...5, 0...2) => &self.cell_blocks[1][(x % 3, y)],
            (6...8, 0...2) => &self.cell_blocks[2][(x % 3, y)],
            (0...2, 3...5) => &self.cell_blocks[3][(x, y % 3)],
            (3...5, 3...5) => &self.cell_blocks[4][(x % 3, y % 3)],
            (6...8, 3...5) => &self.cell_blocks[5][(x % 3, y % 3)],
            (0...2, 6...8) => &self.cell_blocks[6][(x, y % 3)],
            (3...5, 6...8) => &self.cell_blocks[7][(x % 3, y % 3)],
            (6...8, 6...8) => &self.cell_blocks[8][(x % 3, y % 3)],
            _ => panic!("Index values ({}, {}) are out of bounds", x, y)
        }
    }
}

impl IndexMut<(usize, usize)> for Redoku {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Option<CellValue> {
        let (x, y) = index;

        match (x, y) {
            (0...2, 0...2) => &mut self.cell_blocks[0][(x, y)],
            (3...5, 0...2) => &mut self.cell_blocks[1][(x % 3, y)],
            (6...8, 0...2) => &mut self.cell_blocks[2][(x % 3, y)],
            (0...2, 3...5) => &mut self.cell_blocks[3][(x, y % 3)],
            (3...5, 3...5) => &mut self.cell_blocks[4][(x % 3, y % 3)],
            (6...8, 3...5) => &mut self.cell_blocks[5][(x % 3, y % 3)],
            (0...2, 6...8) => &mut self.cell_blocks[6][(x, y % 3)],
            (3...5, 6...8) => &mut self.cell_blocks[7][(x % 3, y % 3)],
            (6...8, 6...8) => &mut self.cell_blocks[8][(x % 3, y % 3)],
            _ => panic!("Index values ({}, {}) are out of bounds", x, y)
        }
    }
}

impl fmt::Debug for Redoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::char;

        let mut string = String::new();

        string.push_str("/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\\");

        for y in 0..9 {
            string.push_str("\n");

            for x in 0..9 {
                string.push_str("|");

                if x == 3 || x == 6 {
                    string.push_str(" |");
                }

                string.push_str(&format!("{}", match self[(x, y)] {
                    Some(val) => char::from_digit(val as u32, 10).unwrap(),
                    None => '?',
                }));
            }

            string.push_str("|");

            if y == 2 || y == 5 {
                string.push_str("\n|                     |");
            }
        }

        string.push_str("\n\\_____________________/");

        write!(f, "{}", string)
    }
}

#[test]
fn test_indexing() {
    let mut redoku = Redoku::new(Difficulty::Easy);

    for x in 0..9 {
        for y in 0..9 {
            assert!(redoku[(x, y)] == None);

            redoku[(x, y)] = Some(CellValue::from_usize(x + 1));
        }
    }

    for block in 0..9 {
        for cell in 0..9 {
            let row = (block % 3) * 3 + cell % 3;

            assert!(redoku.cell_blocks[block].cells[cell].value == Some(CellValue::from_usize(row + 1)));
        }
    }
}

#[test]
fn test_place_if_valid() {
    use self::CellValue::*;

    let mut redoku = Redoku::new(Difficulty::Easy);

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
