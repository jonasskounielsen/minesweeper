use self::cell::{Cell, CellValue};
use crate::helper::PlaceI32;
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
    pub fn new(mine_concentration: f64, seed: u64) -> Grid {
        Self {
            tile: RefCell::new(
                Tile::new(1, CellBuilder::new(mine_concentration, seed)),
            ),
        }
    }

    pub fn get(&self, place: PlaceI32) -> Cell {
        if place.radius() > self.tile.borrow().radius {
            self.tile.borrow_mut().expand();
            self.get(place)
        } else {
            // dbg!(place, self.tile.borrow_mut().get(place).clone()).1
            self.tile.borrow_mut().get(place).clone()
        }
    }

    pub fn get_mut(&mut self, place: PlaceI32) -> RefMut<'_, Cell> {
        if place.radius() > self.tile.borrow().radius {
            self.tile.borrow_mut().expand();
            self.get_mut(place)
        } else {
            let tile = self.tile.borrow_mut();
            RefMut::map(tile, |tile| tile.get(place))
        }
    }
}