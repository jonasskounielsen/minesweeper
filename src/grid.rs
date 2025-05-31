use super::cell::{Cell, CellValue};
use self::tile::{Tile, Subtiles};

mod tile;

pub struct Grid {
    tile: Subtiles,
    // mine_concentration: f32,
}

impl Grid {
    pub fn new(input: Input) -> Grid {
        Self {
            tile: Tile::new(),
        }
    }

    pub fn get(&mut self, place: Place) -> Option<&mut Cell> {
        self.tile.get(place)
    }
}

#[derive(Clone, Copy)]
pub struct Place {
    x: i32,
    y: i32,
}

impl Place {
    const ORIGIN: Place = Place { x: 0, y: 0 };
}

pub struct Input {
    // mine_concentration: f32,
}