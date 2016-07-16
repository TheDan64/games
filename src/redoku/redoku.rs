use std::ops::Index;

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
    Random,
}

#[derive(Debug)]
enum CellValue {
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

#[derive(Debug)]
struct Cell {
    value: Option<CellValue>,
}

#[derive(Debug)]
struct CellBlock {
    cells: [Cell; 9]
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

#[derive(Debug)]
struct Redoku {
    difficulty: Difficulty,
    cell_blocks: [CellBlock; 9],
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
