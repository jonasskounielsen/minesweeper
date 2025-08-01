use std::{io, time};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use crate::grid::cell::{Cell, CellState, CellValue};
use crate::game::{Game, GameState, MineCount};
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
            ViewCell::One   => "1", ViewCell::Two   => "2",
            ViewCell::Three => "3", ViewCell::Four  => "4",
            ViewCell::Five  => "5", ViewCell::Six   => "6",
            ViewCell::Seven => "7", ViewCell::Eight => "8",
        }
    }

    pub fn color(&self, light_mode: bool) -> Color {
        match *self {
            ViewCell::Flagged if light_mode => Color::Rgb { r: 0x00, g: 0x00, b: 0x00 },
            ViewCell::Clear   if light_mode => Color::Rgb { r: 0x00, g: 0x00, b: 0x00 },
            ViewCell::Mine    if light_mode => Color::Rgb { r: 0x00, g: 0x00, b: 0x00 },
            ViewCell::Flagged => Color::Rgb { r: 0xbd, g: 0xbd, b: 0xbd },
            ViewCell::Clear   => Color::Rgb { r: 0xbd, g: 0xbd, b: 0xbd },
            ViewCell::Mine    => Color::Rgb { r: 0xbd, g: 0xbd, b: 0xbd },
            ViewCell::Unrevealed    => Color::Rgb { r: 0x00, g: 0x00, b: 0x00 },
            ViewCell::IncorrectFlag => Color::Rgb { r: 0xff, g: 0x00, b: 0x00 },
            ViewCell::One   => Color::Rgb { r: 0x00, g: 0x00, b: 0xff },
            ViewCell::Two   => Color::Rgb { r: 0x00, g: 0x7b, b: 0x00 },
            ViewCell::Three => Color::Rgb { r: 0xff, g: 0x00, b: 0x00 },
            ViewCell::Four  => Color::Rgb { r: 0x00, g: 0x00, b: 0x7b },
            ViewCell::Five  => Color::Rgb { r: 0x7b, g: 0x00, b: 0x00 },
            ViewCell::Six   => Color::Rgb { r: 0x00, g: 0x7b, b: 0x7b },
            ViewCell::Seven => Color::Rgb { r: 0x00, g: 0x00, b: 0x00 },
            ViewCell::Eight => Color::Rgb { r: 0x7b, g: 0x7b, b: 0x7b },
        }
    }
}

#[derive(Debug)]
pub struct View {
    matrix: Matrix<ViewCell>,
    window_size: SizeUsize,
    window_too_small: bool,
    matrix_cursor: PlaceUsize,
    game_cursor: PlaceI32,
    revealed_cell_count: u32,
    game_duration: time::Duration,
    game_state: GameState,
    seed: u64,
    light_mode: bool,
}

impl View {
    const FOREGROUND_COLOR_LIGHT_MODE: Color = Color::Rgb { r: 0x00, g: 0x00, b: 0x00 };
    const BACKGROUND_COLOR_LIGHT_MODE: Color = Color::Rgb { r: 0xbd, g: 0xbd, b: 0xbd };

    pub fn new(
        grid: &Grid,                        window_size: SizeUsize,
        window_too_small: bool,             origin: PlaceI32,
        game_cursor: PlaceI32,              show_mines: bool,
        revealed_cell_count: u32,           start_instant: time::Instant,
        latest_game_instant: time::Instant, game_state: GameState,
        seed: u64,                          light_mode: bool,
    ) -> View {
        let matrix_size = Self::matrix_size(window_size);
        let matrix = Matrix::new(
            matrix_size,
            |relative: PlaceUsize| {
                let cell_position = PlaceI32 {
                    x: origin.x - matrix_size.width  as i32 / 2 + relative.x as i32,
                    y: origin.y - matrix_size.height as i32 / 2 + relative.y as i32,
                };
                Self::get_view_cell(grid, cell_position, show_mines)
            },
        );
        let matrix_cursor = PlaceUsize {
            x: (game_cursor.x + matrix_size.width  as i32 / 2 - origin.x) as usize,
            y: (game_cursor.y + matrix_size.height as i32 / 2 - origin.y) as usize,
        };
        let game_duration = latest_game_instant.duration_since(start_instant);
        View {
            matrix,
            window_size,
            window_too_small,
            matrix_cursor,
            game_cursor,
            revealed_cell_count,
            game_duration,
            game_state,
            seed,
            light_mode,
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
                value: CellValue::Empty,
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
    // const SLIM_TOP_LEFT_CORNER:     &str = "┌";
    // const SLIM_TOP_RIGHT_CORNER:    &str = "┐";
    // const SLIM_BOTTOM_LEFT_CORNER:  &str = "└";
    // const SLIM_BOTTOM_RIGHT_CORNER: &str = "┘";
    // const SLIM_LEFT_BORDER:         &str = "│";
    // const SLIM_RIGHT_BORDER:        &str = "│";
    // const SLIM_TOP_BORDER:          &str = "─";
    // const SLIM_BOTTOM_BORDER:       &str = "─";
    const SPACE:                    &str = " ";

    pub fn render(&self, buffer: &mut impl io::Write) -> io::Result<()> {
        if self.window_too_small {
            buffer.queue(Clear(ClearType::All))?;
            self.render_line(buffer, 0, "window is too small")?;
            return Ok(());
        }

        let line = if let GameState::Underway = self.game_state {
            format!(
                "{:<pad_dist$}{}",
                "SCORE",
                "TIME",
                pad_dist = self.window_size.width - "TIME".len(),
            )
        } else if self.window_size.width >= 21 {
            let pad_left   = (self.window_size.width - 9) / 2;
            let pad_right  = self.window_size.width - 9 - pad_left;
            format!(
                "{:<pad_left$}{}{:>pad_right$}",
                "SCORE",
                "GAME OVER",
                "TIME",
            )
        } else {
            format!(
                "{:^width$}",
                "GAME OVER",
                width = self.window_size.width,
            )
        };
        self.render_line(buffer, 0, &line)?;

        let time = self.game_duration.as_secs();
        let line = format!(
            "{:<pad_dist$}{time}",
            self.revealed_cell_count.to_string(),
            pad_dist = self.window_size.width - time.to_string().len(),
        );
        self.render_line(buffer, 1, &line)?;

        let mut line = String::new(); 
        line +=  Self::FAT_TOP_LEFT_CORNER;
        line += &Self::FAT_TOP_BORDER.repeat(self.window_size.width - 2);
        line +=  Self::FAT_TOP_RIGHT_CORNER;
        self.render_line(buffer, 2, &line)?;

        for y in (0..self.matrix.size.height).rev() {
            let line_no = self.matrix.size.height - y + 2;
            self.render_character(
                buffer, line_no, 0,
                (Self::FAT_LEFT_BORDER, None),
            )?;

            for x in 0..(self.window_size.width - 2) {
                let place = PlaceUsize { x, y };
                let character_and_color = self.get_character_and_color(place);
                self.render_character(buffer, line_no, x + 1, character_and_color)?;
            }
            self.render_character(
                buffer, line_no, self.window_size.width - 1,
                (Self::FAT_RIGHT_BORDER, None),
            )?;
        }

        let mut line = String::new(); 
        line +=  Self::FAT_BOTTOM_LEFT_CORNER;
        line += &Self::FAT_BOTTOM_BORDER.repeat(self.window_size.width - 2);
        line +=  Self::FAT_BOTTOM_RIGHT_CORNER;
        self.render_line(buffer, self.matrix.size.height + 3, &line)?;

        let line = format!(
            "{:>pad_dist$},{:<pad_dist$}",
            format!("({}", self.game_cursor.x),
            format!("{})", self.game_cursor.y),
            pad_dist = self.window_size.width / 2 - 1,
        );
        self.render_line(buffer, self.matrix.size.height + 4, &line)?;

        let _ = self.seed;
        // let mut line = String::new();
        // line += &format!(
        //     "{:<pad_dist$}0x{:X}",
        //     "SEED",
        //     self.seed,
        //     pad_dist = self.window_size.width - "0x0123456789ABCDEF".len(),
        // );
        // Self::render_line(buffer, self.matrix.size.height + 5, &line)?;

        Ok(())
    }

    fn render_line(&self, buffer: &mut impl io::Write, line: usize, text: &str) -> io::Result<()> {
        buffer.queue(MoveTo(0, line.try_into().expect("line number above u16 integer limit")))?;
        buffer.queue(ResetColor)?;
        if self.light_mode {
            buffer.queue(SetForegroundColor(Self::FOREGROUND_COLOR_LIGHT_MODE))?;
            buffer.queue(SetBackgroundColor(Self::BACKGROUND_COLOR_LIGHT_MODE))?;
        }
        buffer.queue(Print(text))?;
        Ok(())
    }

    fn render_character(
        &self, buffer: &mut impl io::Write, line: usize,
        column: usize, (character, color): (&str, Option<Color>),
    ) -> io::Result<()> {
        buffer.queue(MoveTo(
            column.try_into().expect("column number above u16 integer limit"),
            line  .try_into().expect(  "line number above u16 integer limit"),
        ))?;
        buffer.queue(ResetColor)?;
        if self.light_mode {
            buffer.queue(SetForegroundColor(Self::FOREGROUND_COLOR_LIGHT_MODE))?;
            buffer.queue(SetBackgroundColor(Self::BACKGROUND_COLOR_LIGHT_MODE))?;
        }
        if let Some(color) = color {
            buffer.queue(SetForegroundColor(color))?;
        }
        buffer.queue(Print(character))?;
        Ok(())
    }

    fn get_character_and_color(&self, place: PlaceUsize) -> (&'static str, Option<Color>) {
        let cursor = PlaceUsize {
            x: self.matrix_cursor.x * 2 + 1,
            y: self.matrix_cursor.y,
        };
        let (dist_x, dist_y) = (
            place.x as i32 - cursor.x as i32,
            place.y as i32 - cursor.y as i32,
        );
        match (dist_x, dist_y) {
            // (-1,  1) =>  return Self::SLIM_TOP_LEFT_CORNER,
            // ( 1,  1) =>  return Self::SLIM_TOP_RIGHT_CORNER,
            // (-1, -1) =>  return Self::SLIM_BOTTOM_LEFT_CORNER,
            // ( 1, -1) =>  return Self::SLIM_BOTTOM_RIGHT_CORNER,
            // (-1,  0) =>  return Self::SLIM_LEFT_BORDER,
            // ( 1,  0) =>  return Self::SLIM_RIGHT_BORDER,
            // // top/bottom border would overwrite adjacent cells
            (-1,  0) =>  return ("[", None),
            ( 1,  0) =>  return ("]", None),
            _ => (),
        }

        if place.x % 2 != 1 {
            return (Self::SPACE, None);
        }

        let matrix_place = PlaceUsize {
            x: (place.x - 1) / 2,
            y: place.y
        };

        let view_cell = self.matrix.get(matrix_place);

        (view_cell.char(), Some(view_cell.color(self.light_mode)))
    }

    pub fn matrix_size(window_size: SizeUsize) -> SizeUsize {
        SizeUsize {
            width: (window_size.width - 2) / 2,
            height: window_size.height - 5,
        }
    }
}