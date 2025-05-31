#[derive(Clone)]
pub struct Cell {
    value: CellValue,
    revealed: bool,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self { value, revealed: false }
    }
}

#[derive(Clone)]
pub enum CellValue {
    Mine,
    Empty,
}