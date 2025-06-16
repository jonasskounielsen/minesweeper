use crate::view::View;
use crate::helper::{PlaceI32, SizeI32, SizeUsize};
use crate::grid::Grid;
use crate::grid::cell::{Cell, CellState, CellValue};
use std::time;

pub enum Action {
    MoveCursor(Direction),
    Reveal,
    Flag,
    RevealAdjacent,
    Reset,
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
    mine_concentration: f64,
    seed: u64,
    max_cursor_displacement: SizeI32,
}

impl Game {
    pub fn new(mine_concentration: f64, seed: u64, max_cursor_displacement: SizeI32) -> Game {
        Game {
            state: GameState::Underway,
            grid: Grid::new(mine_concentration, seed),
            cursor: PlaceI32 { x: 0, y: 0 },
            origin: PlaceI32 { x: 0, y: 0 },
            revealed_cell_count: 0,
            start_instant: time::Instant::now(),
            mine_concentration,
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
            _ => (),
        }
    }

    fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Left   => self.cursor.x -= 1,
            Direction::Right  => self.cursor.x += 1,
            Direction::Down   => self.cursor.y -= 1,
            Direction::Up     => self.cursor.y += 1,
        };
        
        let cursor_displacement = i32::abs_diff(
            match direction {
                Direction::Left   => self.cursor.x,
                Direction::Right  => self.cursor.x,
                Direction::Down   => self.cursor.y,
                Direction::Up     => self.cursor.y,
            },
            match direction {
                Direction::Left   => self.origin.x,
                Direction::Right  => self.origin.x,
                Direction::Down   => self.origin.y,
                Direction::Up     => self.origin.y,
            }
        ) as i32;

        let max_displacement = match direction {
            Direction::Left   => self.max_cursor_displacement.width  / 2 + 1,
            Direction::Right  => self.max_cursor_displacement.width  / 2,
            Direction::Down   => self.max_cursor_displacement.height / 2 + 1,
            Direction::Up     => self.max_cursor_displacement.height / 2,
        };

        if cursor_displacement > max_displacement {
            match direction {
                Direction::Left   => self.origin.x -= 1,
                Direction::Right  => self.origin.x += 1,
                Direction::Down   => self.origin.y -= 1,
                Direction::Up     => self.origin.y += 1,
            };
        }
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
            self.state = GameState::Lost;
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
        dbg!("test");
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
        *self = Game {
            state: GameState::Underway,
            grid: Grid::new(self.mine_concentration, self.seed),
            cursor: PlaceI32 { x: 0, y: 0 },
            origin: PlaceI32 { x: 0, y: 0 },
            revealed_cell_count: 0,
            start_instant: time::Instant::now(),
            mine_concentration:      self.mine_concentration,
            seed:                    self.seed,
            max_cursor_displacement: self.max_cursor_displacement,
        };
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

    pub fn view(&self, size: SizeUsize) -> View {
        let show_mines = if let GameState::Lost = self.state { true } else { false };
        View::new(
            &self.grid,  size,
            self.origin, self.cursor,
            show_mines,  self.revealed_cell_count,
            self.start_instant,
        )
    }
}