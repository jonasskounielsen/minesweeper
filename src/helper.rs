use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub struct PlaceI32 {
    pub x: i32,
    pub y: i32,
}

impl PlaceI32 {
    pub const ORIGIN: PlaceI32 = PlaceI32 { x: 0, y: 0 };

    /// Radius of the smallest tile containing the place.
    pub fn radius(&self) -> i32 {
        std::cmp::max(
            if self.x >= 0 { self.x + 1 } else { self.x.abs() },
            if self.y >= 0 { self.y + 1 } else { self.y.abs() },
        )
    }

    pub fn within(&self, place: PlaceI32, area: SizeI32) -> bool {
        if ((place.x - area.width  / 2)..(place.x + area.width  / 2)).contains(&self.x) &&
           ((place.y - area.height / 2)..(place.y + area.height / 2)).contains(&self.y) {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SizeI32 {
    pub width:  i32,
    pub height: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PlaceUsize {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SizeUsize {
    pub width: usize,
    pub height: usize,
}

impl Into<SizeI32> for SizeUsize {
    fn into(self) -> SizeI32 {
        SizeI32 {
            width:  self.width  as i32,
            height: self.height as i32,
        }
    }
}

pub struct Immut<T>(T);

impl<T> Immut<T> {
    pub fn new(value: T) -> Immut<T> {
        Immut(value)
    }
}

impl<T> Deref for Immut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}