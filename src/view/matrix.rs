use super::Size;

#[derive(Debug)]
pub struct Matrix<T> {
    pub size: Size,
    data: Box<[T]>,
}

impl<T> Matrix<T> {
    pub fn new<F: FnMut(Place) -> T>(
        size: Size,
        mut builder: F,
    ) -> Matrix<T> {
        let data =
            (0..size.width).map(
                |x| (0..size.height).map(
                    |y| builder(Place { x: x, y: y }),
                ).collect::<Box<[T]>>(),
            ).flatten().collect();
        Matrix {
            size,
            data,
        }
    }

    pub fn get(&self, place: Place) -> &T {
        &self.data[place.x * self.size.width + place.y]
    }
}

pub struct Place {
    pub x: usize,
    pub y: usize,
}