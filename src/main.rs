pub mod grid;
pub mod view;

use grid::{Grid, Input, Place};
use view::View;

fn main() {
    let mut grid = Grid::new(Input {});

    grid.generate(Place { x: 2, y: 6 }).unwrap();
    grid.generate(Place { x: 3, y: -4 }).unwrap();
    grid.generate(Place { x: -3, y: -6 }).unwrap();
    grid.generate(Place { x: -2, y: 1 }).unwrap();

    grid.get(Place { x: 2, y: 6 });
    grid.get(Place { x: 3, y: -4 });
    grid.get(Place { x: -3, y: -6 });
    grid.get(Place { x: -2, y: 1 });
    
    grid.reveal(Place { x: 3, y: -4 });
    grid.reveal(Place { x: -3, y: -6 });

    let view = View::new(
        &grid, 16, 16,
        Place { x: 0, y: 0 },
    );

    println!("{}", view.as_text());
}