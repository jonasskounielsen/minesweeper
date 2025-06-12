#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub value: CellValue,
    pub state: CellState,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Self { value, state: CellState::Hidden }
    }

    pub fn reveal(&mut self) {
        if let CellState::Hidden = self.state {
            self.state = CellState::Revealed;
        }
    }

    pub fn flag(&mut self) {
        if let CellState::Hidden = self.state {
            self.state = CellState::Flagged;
        }
    }

    pub fn unflag(&mut self) {
        if let CellState::Flagged = self.state {
            self.state = CellState::Hidden;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CellValue {
    Mine,
    Empty,
}

#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Hidden,
    Flagged,
    Revealed,
}