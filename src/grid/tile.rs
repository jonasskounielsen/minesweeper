use crate::cell::CellValue;

use super::{Cell, Place};

#[derive(Debug)]
pub enum Tile {
    Cell(Cell),
    Subtiles(Box<Subtiles>),
    None,
}

#[derive(Debug)]
pub struct Subtiles {
    pub radius: i32,
    pub origin: Place,
    pub top_left:     Tile,
    pub top_right:    Tile,
    pub bottom_left:  Tile,
    pub bottom_right: Tile,
}

#[derive(Clone, Copy, Debug)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Tile {
    pub fn new(radius: i32) -> Subtiles {
        Subtiles {
            origin: Place::ORIGIN,
            radius,
            top_left:     Tile::None,
            top_right:    Tile::None,
            bottom_left:  Tile::None,
            bottom_right: Tile::None,
        }
    }

    pub fn or_none(self) -> Tile {
        match self {
            Tile::Subtiles(tile) => match *tile {
                Subtiles { top_left: Tile::None, top_right: Tile::None, bottom_left: Tile::None, bottom_right: Tile::None, .. } => Tile::None,
                tile => Tile::Subtiles(Box::new(tile)),
            },
            tile => tile,
        }
    }
}

impl Subtiles {
    pub const DUMMY: Subtiles = Subtiles {
        origin: Place::ORIGIN,
        radius: 1,
        top_left:     Tile::None,
        top_right:    Tile::None,
        bottom_left:  Tile::None,
        bottom_right: Tile::None,
    };

    pub fn get(&mut self, place: Place) -> Result<Option<&mut Cell>, &'static str> {
        let subtile = self.subtile(self.quadrant(place)?);

        match subtile {
            Tile::None => Ok(None),
            Tile::Cell(cell) => Ok(Some(cell)),
            Tile::Subtiles(subtile) => subtile.get(place),
        }
    }

    pub fn generate(&mut self, place: Place) -> Result<(), &'static str> {
        let quadrant = self.quadrant(place)?;
        let subtile = self.subtile(quadrant);

        match subtile {
            Tile::None => {
                if self.radius == 1 {
                    self.make_cell(quadrant)
                } else {
                    self.make_tile(quadrant)?;
                    if let Tile::Subtiles(subtile) = self.subtile(quadrant) {
                        subtile.generate(place)
                    } else {
                        unreachable!(); // we just made a subtile there
                    }
                }
            },
            Tile::Cell(_) => Err("cell already exists"),
            Tile::Subtiles(subtile) => subtile.generate(place),
        }
    }

    fn quadrant(&self, place: Place) -> Result<Quadrant, &'static str> {
        let Place { x, y } = place;

        if        self.left()   <= x && x < self.origin.x && self.origin.y <= y && y < self.top() {
            Ok(Quadrant::TopLeft)
        } else if self.origin.x <= x && x < self.right() &&  self.origin.y <= y && y < self.top() {
            Ok(Quadrant::TopRight)
        } else if self.left()   <= x && x < self.origin.x && self.bottom() <= y && y < self.origin.y {
            Ok(Quadrant::BottomLeft)
        } else if self.origin.x <= x && x < self.right() &&  self.bottom() <= y && y < self.origin.y {
            Ok(Quadrant::BottomRight)
        } else {
            Err("invalid place")
        }
    }

    fn subtile(&mut self, quadrant: Quadrant) -> &mut Tile {
        match quadrant {
            Quadrant::TopLeft     => &mut self.top_left,
            Quadrant::TopRight    => &mut self.top_right,
            Quadrant::BottomLeft  => &mut self.bottom_left,
            Quadrant::BottomRight => &mut self.bottom_right,
        }
    }

    fn make_tile(&mut self, quadrant: Quadrant) -> Result<(), &'static str> {
        if self.radius == 1 {
            return Err("tile too small for subtiles");
        }
        if !matches!(self.subtile(quadrant), Tile::None) {
            return Err("quadrant not empty");
        }
        let new_tile = Subtiles {
            radius: self.radius / 2,
            origin: match quadrant {
                Quadrant::TopLeft     => Place { x: self.origin.x - self.radius / 2, y: self.origin.y + self.radius / 2 },
                Quadrant::TopRight    => Place { x: self.origin.x + self.radius / 2, y: self.origin.y + self.radius / 2 },
                Quadrant::BottomLeft  => Place { x: self.origin.x - self.radius / 2, y: self.origin.y - self.radius / 2 },
                Quadrant::BottomRight => Place { x: self.origin.x + self.radius / 2, y: self.origin.y - self.radius / 2 },
            },
            top_left:     Tile::None,
            top_right:    Tile::None,
            bottom_left:  Tile::None,
            bottom_right: Tile::None,
        };
        match quadrant {
            Quadrant::TopLeft     => self.top_left     = Tile::Subtiles(Box::new(new_tile)),
            Quadrant::TopRight    => self.top_right    = Tile::Subtiles(Box::new(new_tile)),
            Quadrant::BottomLeft  => self.bottom_left  = Tile::Subtiles(Box::new(new_tile)),
            Quadrant::BottomRight => self.bottom_right = Tile::Subtiles(Box::new(new_tile)),
        };
        Ok(())
    }

    fn make_cell(&mut self, quadrant: Quadrant) -> Result<(), &'static str> {
        if self.radius != 1 {
            return Err("tile too large");
        }
        if !matches!(self.subtile(quadrant), Tile::None) {
            return Err("quadrant not empty");
        }
        let cell = Cell::new(CellValue::Empty);
        match quadrant {
            Quadrant::TopLeft     => self.top_left     = Tile::Cell(cell),
            Quadrant::TopRight    => self.top_right    = Tile::Cell(cell),
            Quadrant::BottomLeft  => self.bottom_left  = Tile::Cell(cell),
            Quadrant::BottomRight => self.bottom_right = Tile::Cell(cell),
        };
        Ok(())
    }

    pub fn left(&self) -> i32 {
        self.origin.x - self.radius
    }

    pub fn bottom(&self) -> i32 {
        self.origin.y - self.radius
    }

    pub fn right(&self) -> i32 {
        self.origin.x + self.radius
    }

    pub fn top(&self) -> i32 {
        self.origin.y + self.radius
    }
}