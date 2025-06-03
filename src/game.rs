use crate::{grid::{Grid, Place}, view::View};

pub enum Action {
    MoveCursor(Direction),
    Reveal,
    Flag,
}

pub enum  Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Game {
    grid: Grid,
    cursor: Place,
}

impl Game {
    pub fn new(grid: Grid) -> Game {
        Game {
            grid,
            cursor: Place { x: 0, y: 0 },
        }
    }

    // pub fn action(&self, action: Action) {
    //     match action {
    //         Action::Reveal => self.grid.reveal(self.cursor),
    //         Action::Flag => self.grid.
    //     }
    // }

    pub fn view(&self, view_width: usize, view_height: usize) -> View {
        View::new(
            &self.grid,
            view_width,
            view_height,
            self.cursor,
        )
    }
}