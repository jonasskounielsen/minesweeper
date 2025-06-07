pub mod grid;
pub mod view;
pub mod game;

use grid::{Grid, Place};
use view::{View, Size};

fn main() {
    let mut grid = Grid::new(0.8f32, 0xDEADBEEF);

    let view = View::new(
        &grid, Size { width: 4, height: 4 },
        Place { x: 0, y: 0 },
    );

    println!("{}", view.as_text());

    dbg!(grid.get(Place { x: -1, y: 0 }));
    dbg!(grid.get(Place { x: 0, y: 0 }));
    dbg!(grid.get(Place { x: -1, y: -1 }));
    dbg!(grid.get(Place { x: 0, y: -1 }));
}