mod io;
mod input;

use crate::game::input::Input;
use crate::grid::cell_builder::CellBuilder;
use crate::view::View;
use crate::helper::{PlaceI32, SizeI32, SizeUsize};
use crate::grid::Grid;
use crate::grid::cell::{Cell, CellState, CellValue};
use std::sync::mpsc::Sender;
use std::time::{self, Duration};
use clap::Parser;
use crossterm::terminal;
use io::{Io, IoEvent};

pub enum Action {
    MoveCursor(Direction),
    Reveal,
    Flag,
    RevealAdjacent,
    Reset,
    Resize(SizeUsize),
}

pub enum  Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug)]
pub enum MineCount {
    Zero,
    One, Two, Three, Four,
    Five, Six, Seven, Eight,
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Underway,
    Lost,
}

#[derive(Debug)]
pub struct Game {
    state: GameState,
    grid: Grid,
    cursor: PlaceI32,
    origin: PlaceI32,
    revealed_cell_count: u32,
    start_instant: time::Instant,
    end_instant: Option<time::Instant>,
    mine_concentration: f64,
    cell_builder: CellBuilder,
    seed: Option<u64>,
    window_size: SizeUsize,
    light_mode: bool,
    max_cursor_displacement: SizeI32,
    tx_panic: Option<Sender<IoEvent>>,
}

impl Game {
    pub const CURSOR_PADDING: SizeI32 = SizeI32 {
        width:  3,
        height: 3,
    };

    pub fn start() -> std::io::Result<()> {
        let input = Input::parse();
        let window_size = terminal::window_size().expect("failed to get terminal size");
        let window_size: SizeUsize = SizeUsize {
            width:  window_size.columns as usize,
            height: window_size.rows    as usize,
        };
        let mut game = Self::new(
            input.mine_concentration, input.seed,
            window_size, input.light_mode,
            None,
        );
        game.run(std::io::stdout())
    }

    pub fn new(
        mine_concentration: f64,
        seed: Option<u64>,
        window_size: SizeUsize,
        light_mode: bool,
        tx_panic: Option<Sender<IoEvent>>,
    ) -> Game {
        let max_cursor_displacement =
            Self::max_cursor_displacement(window_size);
        let cell_builder =
            CellBuilder::new(
                mine_concentration, seed,
                |message: &'static str| {
                    Self::send_panic(&tx_panic, message);
                },
            );
        let grid = Grid::new(cell_builder);
        let mut game = Game {
            state: GameState::Underway,
            grid,
            cursor: PlaceI32 { x: 0, y: 0 },
            origin: PlaceI32 { x: 0, y: 0 },
            revealed_cell_count: 0,
            start_instant: time::Instant::now(),
            end_instant: None,
            mine_concentration,
            cell_builder,
            seed,
            window_size,
            light_mode,
            max_cursor_displacement,
            tx_panic,
        };
        game.reveal(PlaceI32 { x: 0, y: 0 });
        game
    }

    pub fn run(&mut self, buffer: impl std::io::Write) -> std::io::Result<()> {
        let mut io = Io::new(self, self.window_size);
        io.run(buffer)
    }

    pub fn send_panic(tx_panic: &Option<Sender<IoEvent>>, message: &'static str) {
        if let Some(tx_panic) = tx_panic {
            tx_panic.send(IoEvent::Panic(message)).expect("failed to send io event");
        } else {
            panic!("{}", message);
        }
    }

    pub fn action(&mut self, action: Action) {
        match (self.state, action) {
            (_,                   Action::Resize(new_size)) => self.resize(new_size),
            (_, _) if self.window_too_small(self.window_size) => (),
            (GameState::Underway, Action::Flag)           => self.toggle_flag(self.cursor),
            (GameState::Underway, Action::Reveal)         => self.reveal(self.cursor),
            (GameState::Underway, Action::RevealAdjacent) => self.reveal_adjacent(self.cursor),
            (_,                   Action::MoveCursor(direction)) => self.move_cursor(direction),
            (_,                   Action::Reset) => self.reset(),
            _ => (),
        }
    }

    fn lose(&mut self) {
        self.state = GameState::Lost;
        self.end_instant = Some(time::Instant::now());
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left   => self.cursor.x -= 1,
            Direction::Right  => self.cursor.x += 1,
            Direction::Down   => self.cursor.y -= 1,
            Direction::Up     => self.cursor.y += 1,
        };
        
        self.tether_origin();
    }

    fn toggle_flag(&mut self, place: PlaceI32) {
        let mut cell = self.grid.get_mut(place);
        match cell.state {
            CellState::Hidden => cell.flag(),
            CellState::Flagged => cell.unflag(),
            _ => (),
        }
    }

    fn reveal(&mut self, place: PlaceI32) {
        self.reveal_tracked(place, 0);
    }

    fn reveal_tracked(&mut self, place: PlaceI32, mut revealed: u32) {
        if let CellState::Revealed = self.grid.get(place).state { return };
        
        self.grid.get_mut(place).reveal();

        revealed += 1;
        if revealed >= 10000 {
            // avoid stack overflow
            Self::send_panic(&self.tx_panic, "too many adjacent clear cells; mine concentration is too low");
            return;
        }
        
        if let CellValue::Mine = self.grid.get(place).value {
            self.lose();
            return;
        }
        self.revealed_cell_count += 1;

        let MineCount::Zero = Self::mine_count(&self.grid, place) else { return; };

        for i in -1..=1 {
            for j in -1..=1 {
                if let (0, 0) = (i, j) {
                    continue;
                }
                let place = PlaceI32 { x: place.x + i, y: place.y + j };
                self.reveal_tracked(place, revealed);
            }
        }
    }

    // in original minesweeper, doesn't reveal cells
    // when there are too many flags around the cell 
    fn reveal_adjacent(&mut self, place: PlaceI32) {
        let CellState::Revealed = self.grid.get(place).state else { return; };
        for i in -1..=1 {
            for j in -1..=1 {
                if let (0, 0) = (i, j) {
                    continue;
                }
                let place = PlaceI32 { x: place.x + i, y: place.y + j };

                if let CellState::Flagged = self.grid.get(place).state { continue; }

                self.reveal(place);
            }
        }
    }

    fn reset(&mut self) {
        *self = Game::new(
            self.mine_concentration, self.seed,
            self.window_size,        self.light_mode,
            self.tx_panic.clone(),
        );
    }

    fn resize(&mut self, new_size: SizeUsize) {
        self.window_size = new_size;
        let new_max_cursor_displacement =
            Self::max_cursor_displacement(new_size);
        self.max_cursor_displacement = new_max_cursor_displacement;
        self.tether_cursor();
    }

    fn tether_cursor(&mut self) {
        let cursor_displacement = PlaceI32 {
            x: self.cursor.x - self.origin.x,
            y: self.cursor.y - self.origin.y,
        };

        if cursor_displacement.x >  self.max_cursor_displacement.width / 2 - 1 {
            self.cursor.x = self.origin.x + (self.max_cursor_displacement.width - 1) / 2;
        }

        if cursor_displacement.x < -self.max_cursor_displacement.width / 2 {
            self.cursor.x = self.origin.x -  self.max_cursor_displacement.width / 2;
        }

        if cursor_displacement.y >  self.max_cursor_displacement.height / 2 - 1 {
            self.cursor.y = self.origin.y + (self.max_cursor_displacement.height - 1) / 2;
        }

        if cursor_displacement.y < -self.max_cursor_displacement.height / 2 {
            self.cursor.y = self.origin.y -  self.max_cursor_displacement.height / 2;
        }
    }

    fn tether_origin(&mut self) {
        let cursor_displacement = PlaceI32 {
            x: self.cursor.x - self.origin.x,
            y: self.cursor.y - self.origin.y,
        };

        if cursor_displacement.x >  self.max_cursor_displacement.width / 2 - 1 {
            self.origin.x = self.cursor.x - (self.max_cursor_displacement.width - 1) / 2;
        }

        if cursor_displacement.x < -self.max_cursor_displacement.width / 2 {
            self.origin.x = self.cursor.x +  self.max_cursor_displacement.width / 2;
        }

        if cursor_displacement.y >  self.max_cursor_displacement.height / 2 - 1 {
            self.origin.y = self.cursor.y - (self.max_cursor_displacement.height - 1) / 2;
        }

        if cursor_displacement.y < -self.max_cursor_displacement.height / 2 {
            self.origin.y = self.cursor.y +  self.max_cursor_displacement.height / 2;
        }
    }

    pub fn mine_count(grid: &Grid, place: PlaceI32) -> MineCount {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if let (0, 0) = (i, j) {
                    continue;
                }
                let place = PlaceI32 { x: place.x + i, y: place.y + j }; 
                if let Cell { value: CellValue::Mine, .. } = *grid.get(place) {
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

    pub fn time_until_timer_update() -> time::Duration {
        let elapsed = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("failed to get system time");
        let second = Duration::from_secs(1);
        let remainder = Duration::from_nanos(elapsed.subsec_nanos() as u64);
        second - remainder
    }

    pub fn view(&self) -> View {
        let window_too_small = self.window_too_small(self.window_size);
        let show_mines = if let GameState::Lost = self.state { true } else { false };
        let latest_game_instant = self.end_instant.unwrap_or_else(|| time::Instant::now());
        let game_cursor = self.cursor;
        View::new(
            &self.grid,               self.window_size,
            window_too_small,         self.origin,
            game_cursor,              show_mines,
            self.revealed_cell_count, self.start_instant,
            latest_game_instant,      self.state,
            self.cell_builder.seed,   self.light_mode,
        )
    }

    fn max_cursor_displacement(window_size: SizeUsize) -> SizeI32 {
        let matrix_size = View::matrix_size(window_size);
        SizeI32 {
            width:  matrix_size.width  as i32 - Self::CURSOR_PADDING.width  * 2,
            height: matrix_size.height as i32 - Self::CURSOR_PADDING.height * 2,
        }
    }

    pub fn window_too_small(&self, window_size: SizeUsize) -> bool {
        let matrix_size = View::matrix_size(window_size);
        matrix_size.width  < self.max_cursor_displacement.width  as usize ||
        matrix_size.height < self.max_cursor_displacement.height as usize
    }
}