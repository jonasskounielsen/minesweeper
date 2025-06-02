use crate::grid::cell::{Cell, CellValue};
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
            ViewCell::Unrevealed => " ",
            ViewCell::Flagged    => "+",
            ViewCell::Clear      => "0",
            ViewCell::Mine       => "*",
            ViewCell::One        => "1",
            ViewCell::Two        => "2",
            ViewCell::Three      => "3",
            ViewCell::Four       => "4",
            ViewCell::Five       => "5",
            ViewCell::Six        => "6",
            ViewCell::Seven      => "7",
            ViewCell::Eight      => "8",
        }
    }
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
                let width =      TryInto::<i32>::try_into(width     ).unwrap();
                let height =     TryInto::<i32>::try_into(height    ).unwrap();
                let relative_x = TryInto::<i32>::try_into(relative_x).unwrap();
                let relative_y = TryInto::<i32>::try_into(relative_y).unwrap();
                let cell_position = Place {
                    x: origin.x + width  / 2 - relative_x - 1,
                    y: origin.y + height / 2 - relative_y - 1,
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
            Some(Cell { revealed: false, .. }) | None  => ViewCell::Unrevealed,
            Some(Cell { value: CellValue::Empty, .. }) => ViewCell::Clear,
            Some(Cell { value: CellValue::Mine,  .. }) => ViewCell::Mine,
        }
    }

    pub fn as_text(&self) -> String {
        let mut text = String::new();
        for x in 0..self.height {
            let mut line = String::new();
            for y in 0..self.width {
                line += &self.matrix.get(x, y).char().to_string();
            }
            line += "
            ";
            text += &line;
        }
        text
    }
}