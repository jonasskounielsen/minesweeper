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
                x: cursor.x as usize + size.width  as usize / 2 - origin.x as usize,
                y: cursor.y as usize + size.height as usize / 2 - origin.y as usize,
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

    pub fn render(&self) -> String {
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
                let place = PlaceUsize { x: x, y: y };
                match &self.cursor {
                    Some(cursor) if *cursor == place && Self::draw_cursor() => line += "_",
                    _ => line += &self.matrix.get(place).char().to_string(),
                }
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

    fn draw_cursor() -> bool {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        if duration.as_secs() % 2 == 0 {
            true
        } else {
            false
        }
    }
}