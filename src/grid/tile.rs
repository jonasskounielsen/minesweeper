use super::{Cell, Place};

pub enum Tile {
    Cell(Cell),
    Subtiles(Subtiles),
}

pub struct Subtiles {
    radius: i32,
    origin: Place,
    top_left:     Option<Box<Tile>>,
    top_right:    Option<Box<Tile>>,
    bottom_left:  Option<Box<Tile>>,
    bottom_right: Option<Box<Tile>>,
}

pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Tile {
    pub fn new() -> Subtiles {
        Subtiles {
            origin: Place::ORIGIN,
            radius: 1,
            top_left:     None,
            top_right:    None,
            bottom_left:  None,
            bottom_right: None,
        }
    }
}

impl Subtiles {
    pub fn get(&mut self, place: Place) -> Option<&mut Cell> {
        let subtile = match self.quadrant(place) {
            Quadrant::TopLeft     => &mut self.top_left,
            Quadrant::TopRight    => &mut self.top_right,
            Quadrant::BottomLeft  => &mut self.bottom_left,
            Quadrant::BottomRight => &mut self.bottom_right,
        };

        match subtile {
            None => None,
            Some(boxed) => {
                match &mut **boxed {
                    Tile::Cell(cell) => Some(cell),
                    Tile::Subtiles(subtile) => subtile.get(place),
                }
            }
        }
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

    pub fn quadrant(&self, place: Place) -> Quadrant {
        let Place { x, y } = place;

        if        self.left()   < x && x < self.origin.x && self.origin.y < y && y < self.top() {
            Quadrant::TopLeft
        } else if self.origin.x < x && x < self.right() &&  self.origin.y < y && y < self.top() {
            Quadrant::TopRight
        } else if self.left()   < x && x < self.origin.x && self.bottom() < y && y < self.origin.y {
            Quadrant::BottomLeft
        } else if self.origin.x < x && x < self.right() &&  self.bottom() < y && y < self.origin.y {
            Quadrant::BottomRight
        } else {
            panic!("invalid place");
        }
    }
}