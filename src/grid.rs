use super::cell::Cell;
use self::tile::{Tile, Subtiles};

mod tile;

#[derive(Debug)]
pub struct Grid {
    tile: Subtiles,
    // mine_concentration: f32,
}

impl Grid {
    pub fn new(_input: Input) -> Grid {
        Self {
            tile: Tile::new(),
        }
    }

    pub fn get(&mut self, place: Place) -> Result<Option<&mut Cell>, &'static str> {
        self.tile.get(place)
    }

    pub fn generate(&mut self, place: Place) -> Result<(), &'static str> {
        self.tile.generate(place)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Place {
    pub x: i32,
    pub y: i32,
}

impl Place {
    const ORIGIN: Place = Place { x: 0, y: 0 };
}

pub struct Input {
    // mine_concentration: f32,
}