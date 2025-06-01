pub mod cell;
pub mod grid;

use grid::{Grid, Input, Place};

fn main() {
    let mut grid = Grid::new(Input {});

    dbg!(grid.generate(Place { x: 1, y: 4 }));
    dbg!(grid.generate(Place { x: -3, y: 1 }));
    dbg!(grid.generate(Place { x: 10, y: -5 }));

    dbg!(&grid);
}