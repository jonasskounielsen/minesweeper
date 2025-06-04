use crate::grid::cell::{Cell, CellState, CellValue};
use super::{Grid, Place};
use self::matrix::Matrix;

mod matrix;

#[derive(Debug)]
pub enum ViewCell {
    Unrevealed,
    Flagged,
    Clear,
    Mine,
    One, Two, Three, Four,
    Five, Six, Seven, Eight,
}

impl ViewCell {
    pub const fn char(&self) -> &str {
        match *self {
            ViewCell::Unrevealed => " ", ViewCell::One        => "1", ViewCell::Two        => "2",
            ViewCell::Flagged    => "+", ViewCell::Three      => "3", ViewCell::Four       => "4",
            ViewCell::Clear      => "0", ViewCell::Five       => "5", ViewCell::Six        => "6",
            ViewCell::Mine       => "*", ViewCell::Seven      => "7", ViewCell::Eight      => "8",
        }
    }
}

pub enum MineCount {
    Zero,
    One, Two, Three, Four,
    Five, Six, Seven, Eight,
}

#[derive(Debug)]
pub struct View<'a> {
    pub grid: &'a Grid,
    matrix: Matrix<ViewCell>,
    pub origin: Place,
    width: usize,
    height: usize,
}

impl View<'_> {
    pub fn new(grid: &Grid, width: usize, height: usize, origin: Place) -> View {
        let matrix = Matrix::new(
            width, height,
            |relative_x, relative_y| {
                let cell_position = Place {
                    x: origin.x + width  as i32 / 2 - relative_x as i32 - 1,
                    y: origin.y + height as i32 / 2 - relative_y as i32 - 1,
                };
                View::get_view_cell(grid, cell_position)
            },
        );
        View {
            grid,
            matrix,
            origin,
            width,
            height,
        }
    }

    fn get_view_cell(grid: &Grid, place: Place) -> ViewCell {
        let cell = grid.get(place);
        match cell {
            Cell { state: CellState::Hidden,  .. } => ViewCell::Unrevealed,
            Cell { state: CellState::Flagged, .. } => ViewCell::Flagged,
            Cell { value: CellValue::Mine,    .. } => ViewCell::Mine,
            Cell { value: CellValue::Empty,   .. } => match View::mine_count(grid, place) {
                MineCount::Zero  => ViewCell::Clear,
                MineCount::One   => ViewCell::One,   MineCount::Two   => ViewCell::Two,
                MineCount::Three => ViewCell::Three, MineCount::Four  => ViewCell::Four,
                MineCount::Five  => ViewCell::Five,  MineCount::Six   => ViewCell::Six,
                MineCount::Seven => ViewCell::Seven, MineCount::Eight => ViewCell::Eight,
            },
        }
    }

    fn mine_count(grid: &Grid, place: Place) -> MineCount {
        let mut count = 0;
        for i in -1..1 {
            for j in -1..1 {
                if let (0, 0) = (i, j) {
                    continue;
                }
                if let Cell { value: CellValue::Mine, .. } = grid.get(place) {
                    count += 1;
                }
            }
        }
        match count {
            0 => MineCount::Zero,
            1 => MineCount::One,   2 => MineCount::Two,
            3 => MineCount::Three, 4 => MineCount::Four,
            5 => MineCount::Five,  6 => MineCount::Six,
            7 => MineCount::Seven, 8 => MineCount::Eight,
            _ => unreachable!(), // we only check 8 tiles
        }
    }

    pub fn as_text(&self) -> String {
        let mut text = String::new();
        for x in 0..self.height {
            let mut line = String::new();
            for y in 0..self.width {
                line += &self.matrix.get(x, y).char().to_string();
            }
            line += "\n";
            text += &line;
        }
        text
    }
}