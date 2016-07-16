use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
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

#[derive(Debug, Copy, Clone)]
struct Cell {
    value: Option<CellValue>,
}

impl Cell {
    fn new() -> Cell {
        Cell { value: None }
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

            redoku[(x, y)] = match x {
                0 => Some(CellValue::One),
                1 => Some(CellValue::Two),
                2 => Some(CellValue::Three),
                3 => Some(CellValue::Four),
                4 => Some(CellValue::Five),
                5 => Some(CellValue::Six),
                6 => Some(CellValue::Seven),
                7 => Some(CellValue::Eight),
                8 => Some(CellValue::Nine),
                _ => unreachable!("Logic error")
            };
        }
    }

    // FIXME
    // for block in 0..9 {
    //     for cell in 0..9 {
    //         assert!(redoku.cell_blocks[block].cells[cell].value == Some(CellValue::Four));
    //     }
    // }
}
