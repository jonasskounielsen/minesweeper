use self::cell::{Cell, CellValue};
use self::tile::{Tile, Subtiles};
use std::cell::RefCell;

pub mod cell;
mod tile;

#[derive(Debug)]
pub struct Grid {
    tile: RefCell<Subtiles>,
    mine_concentration: f32,
}

impl Grid {
    pub fn new(mine_concentration: f32) -> Grid {
        Self {
            tile: RefCell::new(
                Tile::new(1, |_| Cell::new(CellValue::Mine)),
            ),
            mine_concentration,
        }
    }

    pub fn get(&self, place: Place) -> Cell {
        if place.radius() > self.tile.radius {
            None
        } else {
            self.tile.get(place)
        }
    }

    pub fn get_mut(&mut self, place: Place) -> &mut Cell {
        if place.radius() > self.tile.borrow().radius {
            self.expand();
            return self.get_mut(place);
        }
        // let mut tile = self.tile.borrow_mut();
        self.tile.borrow_mut().get_mut(place)
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
            builder: old_tile.builder,
            top_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_left, y: old_top },
                top_left:     Tile::None,
                top_right:    Tile::None,
                bottom_left:  Tile::None,
                bottom_right: old_tile.top_left.or_none(),
                builder: old_tile.builder,
            })),
            top_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_right, y: old_top },
                top_left:     Tile::None,
                top_right:    Tile::None,
                bottom_left:  old_tile.top_right.or_none(),
                bottom_right: Tile::None,
                builder: old_tile.builder,
            })),
            bottom_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_left, y: old_bottom },
                top_left:     Tile::None,
                top_right:    old_tile.bottom_left.or_none(),
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
                builder: old_tile.builder,
            })),
            bottom_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_right, y: old_bottom },
                top_left:     old_tile.bottom_right.or_none(),
                top_right:    Tile::None,
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
                builder: old_tile.builder,
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