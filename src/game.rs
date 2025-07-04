use crate::grid::cell_builder::CellBuilder;
use crate::view::View;
use crate::helper::{PlaceI32, SizeI32, SizeUsize};
use crate::grid::Grid;
use crate::grid::cell::{Cell, CellState, CellValue};
use std::time::{self, Duration};

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
enum GameState {
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
    max_cursor_displacement: SizeI32,
}

impl Game {
    pub const CURSOR_PADDING: SizeI32 = SizeI32 {
        width:  3,
        height: 3,
    };

    pub fn new(mine_concentration: f64, seed: Option<u64>, window_size: SizeUsize) -> Game {
        let max_cursor_displacement = Self::max_cursor_displacement(window_size);
        let cell_builder = CellBuilder::new(mine_concentration, seed);
        Game {
            state: GameState::Underway,
            grid: Grid::new(cell_builder),
            cursor: PlaceI32 { x: 0, y: 0 },
            origin: PlaceI32 { x: 0, y: 0 },
            revealed_cell_count: 0,
            start_instant: time::Instant::now(),
            end_instant: None,
            mine_concentration,
            cell_builder,
            seed,
            max_cursor_displacement,
        }
    }

    pub fn action(&mut self, action: Action) {
        match (self.state, action) {
            (GameState::Underway, Action::Flag)           => self.toggle_flag(self.cursor),
            (GameState::Underway, Action::Reveal)         => self.reveal(self.cursor),
            (GameState::Underway, Action::RevealAdjacent) => self.reveal_adjacent(self.cursor),
            (_,                   Action::MoveCursor(direction)) => self.move_cursor(direction),
            (_,                   Action::Reset) => self.reset(),
            (_,                   Action::Resize(new_size)) => self.resize(new_size),
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
        if let CellState::Revealed = self.grid.get(place).state { return };
        
        self.grid.get_mut(place).reveal();
        
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
                self.reveal(place);
            }
        }
    }

    // in original minesweeper, doesn't reveal cells when there are too many flags around the cell 
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
        let cell_builder = CellBuilder::new(self.mine_concentration, self.seed);
        *self = Game {
            state: GameState::Underway,
            grid: Grid::new(cell_builder),
            cursor: PlaceI32 { x: 0, y: 0 },
            origin: PlaceI32 { x: 0, y: 0 },
            revealed_cell_count: 0,
            start_instant: time::Instant::now(),
            end_instant: None,
            mine_concentration:      self.mine_concentration,
            cell_builder,
            seed:                    self.seed,
            max_cursor_displacement: self.max_cursor_displacement,
        };
    }

    fn resize(&mut self, new_size: SizeUsize) {
        let new_max_cursor_displacement = Self::max_cursor_displacement(new_size);
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

    pub fn view(&self, size: SizeUsize) -> View {
        let show_mines = if let GameState::Lost = self.state { true } else { false };
        let latest_game_instant = self.end_instant.unwrap_or_else(|| time::Instant::now());
        View::new(
            &self.grid,         size,
            self.origin,        self.cursor,
            show_mines,         self.revealed_cell_count,
            self.start_instant, latest_game_instant,
            self.cell_builder.seed,
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