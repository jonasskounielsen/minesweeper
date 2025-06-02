use self::cell::Cell;
use self::tile::{Tile, Subtiles};

pub mod cell;
mod tile;

#[derive(Debug)]
pub struct Grid {
    tile: Subtiles,
    // mine_concentration: f32,
}

impl Grid {
    pub fn new(_input: Input) -> Grid {
        Self {
            tile: Tile::new(1),
        }
    }

    pub fn get(&self, place: Place) -> Option<&Cell> {
        if place.radius() > self.tile.radius {
            return None;
        }
        self.tile.get(place).unwrap()
        // we already checked for out of bounds
    }

    fn get_mut(&mut self, place: Place) -> Option<&mut Cell> {
        if place.radius() > self.tile.radius {
            return None;
        }
        self.tile.get_mut(place).unwrap()
        // we already checked for out of bounds
    }

    pub fn generate(&mut self, place: Place) -> Result<(), &'static str> {
        if place.radius() > self.tile.radius {
            self.expand();
            self.generate(place)
        } else {
            self.tile.generate(place)
        }
    }

    pub fn reveal(&mut self, place: Place) {
        match self.get_mut(place) {
            Some(cell) => cell.reveal(),
            None => {
                self.generate(place).unwrap();
                // we already checked if the cell exists
                self.reveal(place);
            },
        }
    }

    fn expand(&mut self) {
        let old_radius = self.tile.radius;
        let old_tile = std::mem::replace(&mut self.tile, Subtiles::DUMMY);
        let old_left = old_tile.left();
        let old_right = old_tile.right();
        let old_top = old_tile.top();
        let old_bottom = old_tile.bottom();

        self.tile = Subtiles {
            radius: old_radius * 2,
            origin: Place::ORIGIN,
            top_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_left, y: old_top },
                top_left:     Tile::None,
                top_right:    Tile::None,
                bottom_left:  Tile::None,
                bottom_right: old_tile.top_left.or_none(),
            })),
            top_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_right, y: old_top },
                top_left:     Tile::None,
                top_right:    Tile::None,
                bottom_left:  old_tile.top_right.or_none(),
                bottom_right: Tile::None,
            })),
            bottom_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_left, y: old_bottom },
                top_left:     Tile::None,
                top_right:    old_tile.bottom_left.or_none(),
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
            })),
            bottom_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_right, y: old_bottom },
                top_left:     old_tile.bottom_right.or_none(),
                top_right:    Tile::None,
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
            })),
        };
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Place {
    pub x: i32,
    pub y: i32,
}

impl Place {
    const ORIGIN: Place = Place { x: 0, y: 0 };

    /// Radius of the smallest tile containing the place.
    pub fn radius(&self) -> i32 {
        std::cmp::max(
            if self.x >= 0 { self.x + 1 } else { self.x.abs() },
            if self.y >= 0 { self.y + 1 } else { self.y.abs() },
        )
    }
}

pub struct Input {
    // mine_concentration: f32,
}