use crate::{grid::{Grid, Place}, view::View};


pub struct Game {
    grid: Grid,
    cursor: Place,
    viewWidth: usize,
    viewHeight: usize,
}

impl Game {
    pub fn new(grid: Grid) -> Game {
        Game {
            grid,
            cursor: Place { x: 0, y: 0 },
        }
    }

    pub fn view(&self) -> View {
        View::new(
            &self.grid, self.viewWidth,
            self.viewHeight, self.cursor,
        )
    }
}