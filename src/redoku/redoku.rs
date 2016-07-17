use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    Evil,
    Random,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Redoku {
    difficulty: Difficulty,
    cell_blocks: [CellBlock; 9],
}

impl Redoku {
    pub fn new() -> Redoku {
        Redoku {
            difficulty: Difficulty::Random,
            cell_blocks: [CellBlock::new(); 9],
        }
    }

    pub fn empty_cells(&self) -> usize {
        // Maybe we can somehow keep track of this on indexes?
        // That way we could get this in constant time (Set.len())

        let mut count = 0;

        for x in 0..9 {
            for y in 0..9 {
                if self[(x, y)] == None {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn is_valid_cell(&self, x: usize, y: usize) -> bool {
        let val = self[(x, y)];

        if val.is_none() {
            println!("Exit 0: {:?}", val);
            return false;
        }

        // See if there is the same value in row and column
        for scanx in 0..9 {
            if scanx == x {
                continue;
            }

            if self[(scanx, y)].is_some() && self[(scanx, y)] == val {
                println!("Exit 1: {},{}:{:?} == {},{}:{:?}", x, y, val, scanx, y, self[(scanx, y)]);
                return false;
            }
        }

        for scany in 0..9 {
            if scany == y {
                continue;
            }

            if self[(x, scany)].is_some() && self[(x, scany)] == val {
                println!("Exit 2");
                return false;
            }
        }

        // See if there is the same value in same cell block
        let (blockx, blocky) = (x / 3, y / 3);
        println!("{},{} belongs in Block: {},{}", x, y, blockx, blocky);
        let (startx, starty) = (blockx * 3, blocky * 3);
        let (endx, endy) = (startx + 3, starty + 3);
        println!("Range: {}-{}, {}-{}", startx, endx, starty, endy);

        for scanx in startx..endx {
            for scany in starty..endy {
                if scanx == x && scany == y {
                    continue;
                }

                println!("Checking Block {}, {}", scanx, scany);

                if self[(scanx, scany)].is_some() && self[(scanx, scany)] == val {
                    println!("Exit 3");
                    return false;
                }
            }
        }

        true
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

#[test]
fn test_indexing() {
    let mut redoku = Redoku::new();

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
fn test_is_valid_cell() {
    use self::CellValue::*;

    let mut redoku = Redoku::new();

    // Test column
    redoku[(1, 1)] = Some(One);

    assert!(redoku.is_valid_cell(1, 1));
    assert!(!redoku.is_valid_cell(8, 1));

    redoku[(8, 1)] = Some(One);

    assert!(!redoku.is_valid_cell(1, 1));
    assert!(!redoku.is_valid_cell(8, 1));

    redoku[(8, 1)] = None;

    assert!(redoku.is_valid_cell(1, 1));

    // Test row
    redoku[(1, 8)] = Some(One);

    assert!(!redoku.is_valid_cell(1, 1));

    redoku[(1, 8)] = None;

    assert!(redoku.is_valid_cell(1, 1));

    // Test block
    redoku[(0, 7)] = Some(One);

    assert!(redoku.is_valid_cell(0, 7));

    redoku[(2, 8)] = Some(One);

    assert!(!redoku.is_valid_cell(0, 7));
    assert!(!redoku.is_valid_cell(2, 8));
}
