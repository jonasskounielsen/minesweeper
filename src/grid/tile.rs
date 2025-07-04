use super::{Cell, PlaceI32, cell_builder::CellBuilder};

#[derive(Debug)]
pub enum Tile {
    Cell(Cell),
    Subtiles(Box<Subtiles>),
    None,
}

#[derive(Debug)]
pub struct Subtiles {
    pub radius: i32,
    pub origin: PlaceI32,
    pub bottom_left:  Tile,
    pub bottom_right: Tile,
    pub top_left:     Tile,
    pub top_right:    Tile,
    pub builder: CellBuilder,
}

#[derive(Clone, Copy, Debug)]
pub enum Quadrant {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl Tile {
    pub fn new(radius: i32, cell_builder: CellBuilder) -> Subtiles {
        Subtiles {
            origin: PlaceI32::ORIGIN,
            radius,
            bottom_left:  Tile::None,
            bottom_right: Tile::None,
            top_left:     Tile::None,
            top_right:    Tile::None,
            builder: cell_builder,
        }
    }

    pub fn or_none(self) -> Tile {
        match self {
            Tile::Subtiles(tile) => match *tile {
                Subtiles {
                    bottom_left:  Tile::None,
                    bottom_right: Tile::None,
                    top_left:     Tile::None,
                    top_right:    Tile::None,
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
        origin: PlaceI32::ORIGIN,
        radius: 1,
        bottom_left:  Tile::None,
        bottom_right: Tile::None,
        top_left:     Tile::None,
        top_right:    Tile::None,
        builder: CellBuilder::DUMMY,
    };
    
    pub fn get(&mut self, place: PlaceI32) -> &mut Cell {
        let quadrant = self.quadrant(place);
        let tile = std::mem::replace(self.subtile(quadrant), Tile::None);

        match tile {
            Tile::None => {
                self.add(place);
                self.get(place)
            },
            Tile::Cell(cell) => {
                *self.subtile(quadrant) = Tile::Cell(cell);
                if let Tile::Cell(cell) = self.subtile(quadrant) {
                    cell
                } else {
                    unreachable!(); // we just set this tile to a cell
                }
            },
            Tile::Subtiles(subtile) => {
                *self.subtile(quadrant) = Tile::Subtiles(subtile);
                if let Tile::Subtiles(subtile) = self.subtile(quadrant) {
                    subtile.get(place)
                } else {
                    unreachable!(); // we just set this tile to subtiles
                }
            },
        }
        
    }

    fn add(&mut self, place: PlaceI32) {
        let radius = self.radius;
        let quadrant = self.quadrant(place);
        let subtile = self.subtile(quadrant);

        match subtile {
            Tile::None if radius == 1 => {
                let cell = self.builder.cell(place);
                match quadrant {
                    Quadrant::BottomLeft  => self.bottom_left  = Tile::Cell(cell),
                    Quadrant::BottomRight => self.bottom_right = Tile::Cell(cell),
                    Quadrant::TopLeft     => self.top_left     = Tile::Cell(cell),
                    Quadrant::TopRight    => self.top_right    = Tile::Cell(cell),
                };
            }
            Tile::None => {
                self.make_tile(quadrant);
                if let Tile::Subtiles(subtile) = self.subtile(quadrant) {
                    subtile.add(place)
                } else {
                    unreachable!(); // we just made a subtile there
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
            origin: PlaceI32::ORIGIN,
            builder: old_tile.builder,

            bottom_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: PlaceI32 { x: old_left, y: old_bottom },
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
                top_left:     Tile::None,
                top_right:    old_tile.bottom_left.or_none(),
                builder: old_tile.builder,
            })),

            bottom_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: PlaceI32 { x: old_right, y: old_bottom },
                bottom_left:  Tile::None,
                bottom_right: Tile::None,
                top_left:     old_tile.bottom_right.or_none(),
                top_right:    Tile::None,
                builder: old_tile.builder,
            })),

            top_left: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: PlaceI32 { x: old_left, y: old_top },
                bottom_left:  Tile::None,
                bottom_right: old_tile.top_left.or_none(),
                top_left:     Tile::None,
                top_right:    Tile::None,
                builder: old_tile.builder,
            })),

            top_right: Tile::Subtiles(Box::new(Subtiles {
                radius: old_radius,
                origin: PlaceI32 { x: old_right, y: old_top },
                bottom_left:  old_tile.top_right.or_none(),
                bottom_right: Tile::None,
                top_left:     Tile::None,
                top_right:    Tile::None,
                builder: old_tile.builder,
            })),

        };
    }

    fn quadrant(&self, place: PlaceI32) -> Quadrant {
        let PlaceI32 { x, y } = place;

        if        self.left()   <= x && x < self.origin.x && self.bottom() <= y && y < self.origin.y {
            Quadrant::BottomLeft
        } else if self.origin.x <= x && x < self.right()  && self.bottom() <= y && y < self.origin.y {
            Quadrant::BottomRight
        } else if self.left()   <= x && x < self.origin.x && self.origin.y <= y && y < self.top()    {
            Quadrant::TopLeft
        } else if self.origin.x <= x && x < self.right()  && self.origin.y <= y && y < self.top()    {
            Quadrant::TopRight
        } else {
            panic!("invalid place");
        }
    }

    fn subtile(&mut self, quadrant: Quadrant) -> &mut Tile {
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
        if !matches!(self.subtile(quadrant), Tile::None) {
            panic!("quadrant not empty");
        }
        let new_tile = Subtiles {
            radius: self.radius / 2,
            origin: match quadrant {
                Quadrant::BottomLeft  => PlaceI32 { x: self.origin.x - self.radius / 2, y: self.origin.y - self.radius / 2 },
                Quadrant::BottomRight => PlaceI32 { x: self.origin.x + self.radius / 2, y: self.origin.y - self.radius / 2 },
                Quadrant::TopLeft     => PlaceI32 { x: self.origin.x - self.radius / 2, y: self.origin.y + self.radius / 2 },
                Quadrant::TopRight    => PlaceI32 { x: self.origin.x + self.radius / 2, y: self.origin.y + self.radius / 2 },
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