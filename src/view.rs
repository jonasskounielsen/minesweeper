use std::time;

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
    IncorrectFlag,
    One, Two, Three, Four,
    Five, Six, Seven, Eight,
}

impl ViewCell {
    pub const fn char(&self) -> &'static str {
        match self {
            ViewCell::Unrevealed    => " ",
            ViewCell::Flagged       => "+",
            ViewCell::Clear         => "0",
            ViewCell::Mine          => "*",
            ViewCell::IncorrectFlag => "X",
            ViewCell::One        => "1", ViewCell::Two        => "2",
            ViewCell::Three      => "3", ViewCell::Four       => "4",
            ViewCell::Five       => "5", ViewCell::Six        => "6",
            ViewCell::Seven      => "7", ViewCell::Eight      => "8",
        }
    }
}

#[derive(Debug)]
pub struct View {
    matrix: Matrix<ViewCell>,
    size: SizeUsize,
    matrix_cursor: PlaceUsize,
    game_cursor: PlaceI32,
    revealed_cell_count: u32,
    start_instant: time::Instant,
}

impl View {
    pub fn new(
        grid: &Grid,      size: SizeUsize,
        origin: PlaceI32, game_cursor: PlaceI32,
        show_mines: bool, revealed_cell_count: u32,
        start_instant: time::Instant,
    ) -> View {
        let matrix = Matrix::new(
            size,
            |relative: PlaceUsize| {
                let cell_position = PlaceI32 {
                    x: origin.x - size.width  as i32 / 2 + relative.x as i32,
                    y: origin.y - size.height as i32 / 2 + relative.y as i32,
                };
                Self::get_view_cell(grid, cell_position, show_mines)
            },
        );
        let matrix_cursor = PlaceUsize {
            x: (game_cursor.x + size.width  as i32 / 2 - origin.x) as usize,
            y: (game_cursor.y + size.height as i32 / 2 - origin.y) as usize,
        };
        View {
            matrix,
            size,
            matrix_cursor,
            game_cursor,
            revealed_cell_count,
            start_instant,
        }
    }

    fn get_view_cell(grid: &Grid, place: PlaceI32, show_mines: bool) -> ViewCell {
        let cell = grid.get(place);
        match *cell {
            Cell {
                state: CellState::Hidden,
                value: CellValue::Mine,
            } if show_mines => ViewCell::Mine,
            Cell {
                state: CellState::Flagged,
                value: CellValue::Mine,
            } if show_mines => ViewCell::IncorrectFlag,
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
    // const SLIM_TOP_BORDER:          &str = "─";
    // const SLIM_BOTTOM_BORDER:       &str = "─";
    const SPACE:                    &str = " ";

    pub fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();

        let mut line = String::new(); 
        line += &format!(
            "{:<pad_dist$}{}",
            "SCORE",
            "TIME",
            pad_dist = self.size.width * 2 - 1,
        );
        lines.push(line);

        let mut line = String::new(); 
        let time = self.start_instant.elapsed().as_secs();
        line += &format!(
            "{:<pad_dist$}{time}",
            self.revealed_cell_count.to_string(),
            pad_dist = self.size.width * 2 + 3 - time.to_string().len(),
        );
        lines.push(line);

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

        let mut line = String::new(); 
        line += &format!(
            "{:>pad_dist$},{:<pad_dist$}",
            format!("({}", self.game_cursor.x),
            format!("{})", self.game_cursor.y),
            pad_dist = self.size.width + 1,
        );
        lines.push(line);

        lines
    }

    fn get_character(&self, place: PlaceUsize) -> &'static str {
        let matrix_cursor = self.matrix_cursor;
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