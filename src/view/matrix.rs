#[derive(Debug)]
pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    data: Box<[T]>,
}

impl<T> Matrix<T> {
    pub fn new<F: FnMut(usize, usize) -> T>(
        width: usize,
        height: usize,
        mut builder: F,
    ) -> Matrix<T> {
        let data =
            (0..width).map(
                |x| (0..height).map(
                    |y| builder(x, y),
                ).collect::<Box<[T]>>(),
            ).flatten().collect();
        Matrix {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x * self.width + y]
    }
}