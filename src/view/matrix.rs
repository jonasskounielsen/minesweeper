struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new<F: FnMut() -> T>(
        width: usize,
        height: usize,
        mut builder: F,
    ) -> Matrix<T> {
        let data = (0..width)
            .map(
                |_| (0..height).map(|_| builder()).collect()
            ).collect();
        Matrix {
            width,
            height,
            data,
        }
    }
}