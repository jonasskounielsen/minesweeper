use crate::view::View;
use crate::helper::{SizeUsize, PlaceI32};
use crate::grid::Grid;
use crate::grid::cell::{Cell, CellValue};

pub enum Action {
    MoveCursor(Direction),
    Reveal,
    Flag,
    RevealAdjacent,
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

#[derive(Debug)]
pub struct Game {
    grid: Grid,
    cursor: PlaceI32,
}

impl Game {
    pub fn new(grid: Grid) -> Game {
        Game {
            grid,
            cursor: PlaceI32 { x: 0, y: 0 },
        }
    }

    pub fn action(&mut self, action: Action) {
        match action {
            Action::Flag           => self.grid.get_mut(self.cursor).flag(),
            Action::Reveal         => self.reveal(self.cursor),
            Action::RevealAdjacent => self.reveal_adjacent(self.cursor),
            Action::MoveCursor(direction) => {
                match direction {
                    Direction::Left   => self.cursor.x -= 1,
                    Direction::Right  => self.cursor.x += 1,
                    Direction::Down   => self.cursor.y -= 1,
                    Direction::Up     => self.cursor.y += 1,
                }
            },
        }
    }

    fn reveal(&mut self, place: PlaceI32) {
        self.grid.get_mut(place).reveal();
        if let MineCount::Zero = Self::mine_count(&self.grid, self.cursor) {
            for i in -1..=1 {
                for j in -1..=1 {
                    if let (0, 0) = (i, j) {
                        continue;
                    }
                    let place = PlaceI32 { x: self.cursor.x + i, y: self.cursor.y + j };
                    self.reveal(place);
                }
            }
        }
    }

    fn reveal_adjacent(&mut self, place: PlaceI32) {
        if let MineCount::Zero = Self::mine_count(&self.grid, place) {
            for i in -1..=1 {
                for j in -1..=1 {
                    if let (0, 0) = (i, j) {
                        continue;
                    }
                    let place = PlaceI32 { x: self.cursor.x + i, y: self.cursor.y + j };
                    self.reveal(place);
                }
            }
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

    pub fn view(&self, size: SizeUsize) -> View {
        View::new(
            &self.grid,
            size,
            self.cursor,
            self.cursor,
        )
    }
}