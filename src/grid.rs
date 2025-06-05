use self::cell::{Cell, CellValue};
use self::cell_builder::CellBuilder;
use self::tile::{Tile, Subtiles};
use std::cell::{RefCell, RefMut};

pub mod cell;
mod cell_builder;
mod tile;

#[derive(Debug)]
pub struct Grid {
    tile: RefCell<Subtiles>,
}

impl Grid {
    pub fn new(mine_concentration: f32, seed: u64) -> Grid {
        Self {
            tile: RefCell::new(
                Tile::new(1, CellBuilder::new(mine_concentration, seed)),
            ),
        }
    }

    pub fn get(&self, place: Place) -> Cell {
        if place.radius() > self.tile.borrow().radius {
            self.tile.borrow_mut().expand();
            self.get(place)
        } else {
            self.tile.borrow_mut().get(place).clone()
        }
    }

    pub fn get_mut(&mut self, place: Place) -> RefMut<'_, Cell> {
        if place.radius() > self.tile.borrow().radius {
            self.tile.borrow_mut().expand();
            self.get_mut(place)
        } else {
            let tile = self.tile.borrow_mut();
            RefMut::map(tile, |tile| tile.get(place))
        }
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