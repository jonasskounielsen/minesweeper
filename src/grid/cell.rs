#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub value: CellValue,
    pub revealed: bool,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self { value, revealed: false }
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CellValue {
    Mine,
    Empty,
}