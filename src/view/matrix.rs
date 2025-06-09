use crate::helper::{SizeUsize, PlaceUsize};

#[derive(Debug)]
pub struct Matrix<T> {
    pub size: SizeUsize,
    data: Box<[T]>,
}

impl<T> Matrix<T> {
    pub fn new<F: FnMut(PlaceUsize) -> T>(
        size: SizeUsize,
        mut builder: F,
    ) -> Matrix<T> {
        let data =
            (0..size.width).map(
                |x| (0..size.height).map(
                    |y| builder(PlaceUsize { x: x, y: y }),
                ).collect::<Box<[T]>>(),
            ).flatten().collect();
        Matrix {
            size,
            data,
        }
    }

    pub fn get(&self, place: PlaceUsize) -> &T {
        &self.data[place.x * self.size.width + place.y]
    }
}