pub mod grid;
pub mod view;
pub mod game;

use grid::{Grid, Place};
use view::{View, Size};

fn main() {
    let /* mut */ grid = Grid::new(0.3f32, 0xDEADBEEF);

    let view = View::new(
        &grid, Size { width: 16, height: 16 },
        Place { x: 0, y: 0 },
    );

    println!("{}", view.as_text());
}