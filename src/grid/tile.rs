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
    pub bottom_left:  Tile,
    pub bottom_right: Tile,
    pub top_left:     Tile,
    pub top_right:    Tile,
    pub builder: fn(Place) -> Cell,
}

#[derive(Clone, Copy, Debug)]
pub enum Quadrant {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl Tile {
    pub fn new(radius: i32, builder: fn(Place) -> Cell) -> Subtiles {
        Subtiles {
            origin: Place::ORIGIN,
            radius,
            bottom_left:  Tile::None,
            bottom_right: Tile::None,
            top_left:     Tile::None,
            top_right:    Tile::None,
            builder,
        }
    }

    pub fn or_none(self) -> Tile {
        match self {
            Tile::Subtiles(tile) => match *tile {
                Subtiles {
                    bottom_left: Tile::None,
                    bottom_right: Tile::None,
                    top_left: Tile::None,
                    top_right: Tile::None,
                    ..
                } => Tile::None,
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
        bottom_left:  Tile::None,
        bottom_right: Tile::None,
        top_left:     Tile::None,
        top_right:    Tile::None,
        builder: |_| unimplemented!("dummy struct"),
    };
    
    pub fn get(&mut self, place: Place) -> &mut Cell {
        let quadrant = self.quadrant(place);
        let tile = std::mem::replace(self.subtile_mut(quadrant), Tile::None);

        match tile {
            Tile::None => {
                self.add(place);
                self.get(place)
            },
            Tile::Cell(cell) => {
                *self.subtile_mut(quadrant) = Tile::Cell(cell);
                match self.subtile_mut(quadrant) {
                    Tile::Cell(cell) => cell,
                    _ => unreachable!(), // we just set this tile to a cell
                }
            },
            Tile::Subtiles(subtile) => {
                *self.subtile_mut(quadrant) = Tile::Subtiles(subtile);
                match self.subtile_mut(quadrant) {
                    Tile::Subtiles(subtile) => subtile.get(place),
                    _ => unreachable!(), // we just set this tile to subtiles
                }
            },
        }
        
    }

    fn add(&mut self, place: Place) {
        let quadrant = self.quadrant(place);
        let subtile = self.subtile_mut(quadrant);

        match subtile {
            Tile::None => {
                if self.radius == 1 {
                    let cell = (self.builder)(place);
                    match quadrant {
                        Quadrant::BottomLeft  => self.bottom_left  = Tile::Cell(cell),
                        Quadrant::BottomRight => self.bottom_right = Tile::Cell(cell),
                        Quadrant::TopLeft     => self.top_left     = Tile::Cell(cell),
                        Quadrant::TopRight    => self.top_right    = Tile::Cell(cell),
                    };
                } else {
                    self.make_tile(quadrant);
                    if let Tile::Subtiles(subtile) = self.subtile_mut(quadrant) {
                        subtile.add(place)
                    } else {
                        unreachable!(); // we just made a subtile there
                    }
                }
            },
            Tile::Cell(_) => panic!("cell already exists"),
            Tile::Subtiles(subtile) => subtile.add(place),
        }
    }

    pub fn expand(&mut self) {
        let old_radius = self.radius;
        let old_tile = std::mem::replace(self, Subtiles::DUMMY);
        let old_left   = old_tile.left();
        let old_right  = old_tile.right();
        let old_bottom = old_tile.bottom();
        let old_top    = old_tile.top();

        *self = Subtiles {
            radius: old_radius * 2,
            origin: Place::ORIGIN,
            builder: old_tile.builder,
            bottom_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_left, y: old_bottom },
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
                top_left:     Tile::None,
                top_right:    old_tile.bottom_left.or_none(),
                builder: old_tile.builder,
            })),
            bottom_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_right, y: old_bottom },
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
                top_left:     old_tile.bottom_right.or_none(),
                top_right:    Tile::None,
                builder: old_tile.builder,
            })),
            top_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_left, y: old_top },
                bottom_left:  Tile::None,
                bottom_right: old_tile.top_left.or_none(),
                top_left:     Tile::None,
                top_right:    Tile::None,
                builder: old_tile.builder,
            })),
            top_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: Place { x: old_right, y: old_top },
                bottom_left:  old_tile.top_right.or_none(),
                bottom_right: Tile::None,
                top_left:     Tile::None,
                top_right:    Tile::None,
                builder: old_tile.builder,
            })),
        };
    }

    fn quadrant(&self, place: Place) -> Quadrant {
        let Place { x, y } = place;

        if        self.left()   <= x && x < self.origin.x && self.bottom() <= y && y < self.origin.y {
            Quadrant::BottomLeft
        } else if self.origin.x <= x && x < self.right()  && self.bottom() <= y && y < self.origin.y {
            Quadrant::BottomRight
        } else if self.left()   <= x && x < self.origin.x && self.origin.y <= y && y < self.top() {
            Quadrant::TopLeft
        } else if self.origin.x <= x && x < self.right()  && self.origin.y <= y && y < self.top() {
            Quadrant::TopRight
        } else {
            panic!("invalid place");
        }
    }

    fn subtile(&self, quadrant: Quadrant) -> &Tile {
        match quadrant {
            Quadrant::BottomLeft  => &self.bottom_left,
            Quadrant::BottomRight => &self.bottom_right,
            Quadrant::TopLeft     => &self.top_left,
            Quadrant::TopRight    => &self.top_right,
        }
    }

    fn subtile_mut(&mut self, quadrant: Quadrant) -> &mut Tile {
        match quadrant {
            Quadrant::BottomLeft  => &mut self.bottom_left,
            Quadrant::BottomRight => &mut self.bottom_right,
            Quadrant::TopLeft     => &mut self.top_left,
            Quadrant::TopRight    => &mut self.top_right,
        }
    }

    fn make_tile(&mut self, quadrant: Quadrant) {
        if self.radius == 1 {
            panic!("tile too small for subtiles");
        }
        if !matches!(self.subtile_mut(quadrant), Tile::None) {
            panic!("quadrant not empty");
        }
        let new_tile = Subtiles {
            radius: self.radius / 2,
            origin: match quadrant {
                Quadrant::BottomLeft  => Place { x: self.origin.x - self.radius / 2, y: self.origin.y - self.radius / 2 },
                Quadrant::BottomRight => Place { x: self.origin.x + self.radius / 2, y: self.origin.y - self.radius / 2 },
                Quadrant::TopLeft     => Place { x: self.origin.x - self.radius / 2, y: self.origin.y + self.radius / 2 },
                Quadrant::TopRight    => Place { x: self.origin.x + self.radius / 2, y: self.origin.y + self.radius / 2 },
            },
            bottom_left:  Tile::None,
            bottom_right: Tile::None,
            top_left:     Tile::None,
            top_right:    Tile::None,
            builder: self.builder,
        };
        match quadrant {
            Quadrant::BottomLeft  => self.bottom_left  = Tile::Subtiles(Box::new(new_tile)),
            Quadrant::BottomRight => self.bottom_right = Tile::Subtiles(Box::new(new_tile)),
            Quadrant::TopLeft     => self.top_left     = Tile::Subtiles(Box::new(new_tile)),
            Quadrant::TopRight    => self.top_right    = Tile::Subtiles(Box::new(new_tile)),
        };
    }

    pub fn left(&self) -> i32 {
        self.origin.x - self.radius
    }
    
    pub fn right(&self) -> i32 {
        self.origin.x + self.radius
    }

    pub fn bottom(&self) -> i32 {
        self.origin.y - self.radius
    }
    
    pub fn top(&self) -> i32 {
        self.origin.y + self.radius
    }
}