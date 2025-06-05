use super::Size;

#[derive(Debug)]
pub struct Matrix<T> {
    pub size: Size,
    data: Box<[T]>,
}

impl<T> Matrix<T> {
    pub fn new<F: FnMut(usize, usize) -> T>(
        size: Size,
        mut builder: F,
    ) -> Matrix<T> {
        let data =
            (0..size.width).map(
                |x| (0..size.height).map(
                    |y| builder(x, y),
                ).collect::<Box<[T]>>(),
            ).flatten().collect();
        Matrix {
            size,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x * self.size.width + y]
    }
}