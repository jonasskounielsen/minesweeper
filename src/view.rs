use crossterm::cursor;

use crate::grid::cell::{Cell, CellState, CellValue};
use crate::game::{Game, MineCount};
use crate::grid::Grid;
use self::matrix::Matrix;
use crate::helper::{PlaceI32, PlaceUsize, SizeUsize};

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
    pub const fn char(&self) -> &'static str {
        match *self {
            ViewCell::Unrevealed => " ", ViewCell::One        => "1", ViewCell::Two        => "2",
            ViewCell::Flagged    => "+", ViewCell::Three      => "3", ViewCell::Four       => "4",
            ViewCell::Clear      => "0", ViewCell::Five       => "5", ViewCell::Six        => "6",
            ViewCell::Mine       => "*", ViewCell::Seven      => "7", ViewCell::Eight      => "8",
        }
    }
}

#[derive(Debug)]
pub struct View {
    matrix: Matrix<ViewCell>,
    size: SizeUsize,
    cursor: Option<PlaceUsize>,
}

impl View {
    pub fn new(grid: &Grid, size: SizeUsize, origin: PlaceI32, cursor: PlaceI32) -> View {
        let matrix = Matrix::new(
            size,
            |relative: PlaceUsize| {
                let cell_position = PlaceI32 {
                    x: origin.x - size.width  as i32 / 2 + relative.x as i32,
                    y: origin.y - size.height as i32 / 2 + relative.y as i32,
                };
                Self::get_view_cell(grid, cell_position)
            },
        );
        let cursor = if cursor.within(origin, size.into()) {
            Some(PlaceUsize {
                x: (cursor.x + size.width  as i32 / 2 - origin.x) as usize,
                y: (cursor.y + size.height as i32 / 2 - origin.y) as usize,
            })
        } else { None };
        View {
            matrix,
            size,
            cursor: cursor,
        }
    }

    fn get_view_cell(grid: &Grid, place: PlaceI32) -> ViewCell {
        let cell = grid.get(place);
        match *cell {
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

    // characters from https://en.wikipedia.org/wiki/Box-drawing_characters
    const FAT_TOP_LEFT_CORNER:      &str = "┏";
    const FAT_TOP_RIGHT_CORNER:     &str = "┓";
    const FAT_BOTTOM_LEFT_CORNER:   &str = "┗";
    const FAT_BOTTOM_RIGHT_CORNER:  &str = "┛";
    const FAT_LEFT_BORDER:          &str = "┃";
    const FAT_RIGHT_BORDER:         &str = "┃";
    const FAT_TOP_BORDER:           &str = "━";
    const FAT_BOTTOM_BORDER:        &str = "━";
    const SLIM_TOP_LEFT_CORNER:     &str = "┌";
    const SLIM_TOP_RIGHT_CORNER:    &str = "┐";
    const SLIM_BOTTOM_LEFT_CORNER:  &str = "└";
    const SLIM_BOTTOM_RIGHT_CORNER: &str = "┘";
    const SLIM_LEFT_BORDER:         &str = "│";
    const SLIM_RIGHT_BORDER:        &str = "│";
    const SLIM_TOP_BORDER:          &str = "─";
    const SLIM_BOTTOM_BORDER:       &str = "─";
    const SPACE:                    &str = " ";

    pub fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();

        let mut line = String::new(); 
        line +=  Self::FAT_TOP_LEFT_CORNER;
        line += &Self::FAT_TOP_BORDER.repeat(self.size.width * 2 + 1);
        line +=  Self::FAT_TOP_RIGHT_CORNER;
        lines.push(line);

        for y in (0..self.size.height).rev() {
            let mut line = String::new();

            line += Self::FAT_LEFT_BORDER;
            for x in 0..(self.size.width * 2 + 1) {
                let place = PlaceUsize { x, y };
                line += self.get_character(place);
            }
            line += Self::FAT_RIGHT_BORDER;
            lines.push(line);
        }

        let mut line = String::new(); 
        line +=  Self::FAT_BOTTOM_LEFT_CORNER;
        line += &Self::FAT_BOTTOM_BORDER.repeat(self.size.width * 2 + 1);
        line +=  Self::FAT_BOTTOM_RIGHT_CORNER;
        lines.push(line);

        lines
    }

    fn get_character(&self, place: PlaceUsize) -> &'static str {
        if let Some(matrix_cursor) = &self.cursor {
            let cursor = PlaceUsize {
                x: matrix_cursor.x * 2 + 1,
                y: matrix_cursor.y,
            };
            let (dist_x, dist_y) = (
                place.x as i32 - cursor.x as i32,
                place.y as i32 - cursor.y as i32,
            );
            match (dist_x, dist_y) {
                (-1,  1) =>  return Self::SLIM_TOP_LEFT_CORNER,
                ( 1,  1) =>  return Self::SLIM_TOP_RIGHT_CORNER,
                (-1, -1) =>  return Self::SLIM_BOTTOM_LEFT_CORNER,
                ( 1, -1) =>  return Self::SLIM_BOTTOM_RIGHT_CORNER,
                (-1,  0) =>  return Self::SLIM_LEFT_BORDER,
                ( 1,  0) =>  return Self::SLIM_RIGHT_BORDER,
                // top/bottom border would overwrite adjacent cells
                _ => (),
            }
        }

        if place.x % 2 != 1 {
            return Self::SPACE;
        };

        let matrix_place = PlaceUsize {
            x: (place.x - 1) / 2,
            y: place.y
        };

        self.matrix.get(matrix_place).char()
    }
}