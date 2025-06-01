pub struct Cell {
    value: CellValue,
    revealed: bool,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self { value, revealed: false }
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }
}

pub enum CellValue {
    Mine,
    Empty,
}