pub mod grid;
pub mod view;
pub mod game;

use grid::{Grid, Place};
use view::{View, Size};

fn main() {
    let mut grid = Grid::new(0.3f32);

    grid.get_mut(Place { x: 2, y: 6 });
    grid.get_mut(Place { x: 3, y: -4 });
    grid.get_mut(Place { x: -3, y: -6 });
    grid.get_mut(Place { x: -2, y: 1 });

    grid.get(Place { x: 2, y: 6 });
    grid.get(Place { x: 3, y: -4 });
    grid.get(Place { x: -3, y: -6 });
    grid.get(Place { x: -2, y: 1 });
    
    grid.get_mut(Place { x: 3, y: -4 }).reveal();
    grid.get_mut(Place { x: -3, y: -6 }).reveal();

    let view = View::new(
        &grid, Size { width: 16, height: 16 },
        Place { x: 0, y: 0 },
    );

    println!("{}", view.as_text());
}