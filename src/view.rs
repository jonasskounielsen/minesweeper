use crate::grid::cell::{Cell, CellState, CellValue};
use crate::game::{Game, MineCount};
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

#[derive(Debug)]
pub struct View<'a> {
    pub grid: &'a Grid,
    matrix: Matrix<ViewCell>,
    pub origin: Place,
    size: Size,
}

impl View<'_> {
    pub fn new(grid: &Grid, size: Size, origin: Place) -> View {
        let matrix = Matrix::new(
            size,
            |relative_x, relative_y| {
                let cell_position = Place {
                    x: origin.x - size.width  as i32 / 2 + relative_x as i32,
                    y: origin.y - size.height as i32 / 2 + relative_y as i32,
                };
                Self::get_view_cell(grid, cell_position)
            },
        );
        View {
            grid,
            matrix,
            origin,
            size,
        }
    }

    fn get_view_cell(grid: &Grid, place: Place) -> ViewCell {
        let cell = grid.get(place);
        match cell {
            Cell { state: CellState::Hidden,  .. } => ViewCell::Unrevealed,
            Cell { state: CellState::Flagged, .. } => ViewCell::Flagged,
            Cell { value: CellValue::Mine,    .. } => ViewCell::Mine,
            Cell { value: CellValue::Empty,   .. } => match Game::mine_count(grid, place) {
                MineCount::Zero  => ViewCell::Clear,
                MineCount::One   => ViewCell::One,   MineCount::Two   => ViewCell::Two,
                MineCount::Three => ViewCell::Three, MineCount::Four  => ViewCell::Four,
                MineCount::Five  => ViewCell::Five,  MineCount::Six   => ViewCell::Six,
                MineCount::Seven => ViewCell::Seven, MineCount::Eight => ViewCell::Eight,
            },
        }
    }

    pub fn as_text(&self) -> String {
        // characters from https://en.wikipedia.org/wiki/Box-drawing_characters
        let mut text = String::new();

        text += "\u{250F}";
        text += &"\u{2501}".repeat(self.size.width * 2 + 1);
        text += "\u{2513}";
        text += "\n";

        for y in (0..self.size.height).rev() {
            let mut line = String::new();
            line += "\u{2503}";
            line += " ";
            for x in 0..self.size.width {
                line += &self.matrix.get(x, y).char().to_string();
                line += " "; // terminal characters are approx. half a sqaure horizontally
            }
            line += "\u{2503}";
            line += "\n";
            text += &line;
        }

        text += "\u{2517}";
        text += &"\u{2501}".repeat(self.size.width * 2 + 1);
        text += "\u{251B}";
        text += "\n";

        text
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}