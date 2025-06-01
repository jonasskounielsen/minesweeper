pub mod cell;
pub mod grid;

use grid::{Grid, Input, Place};

fn main() {
    let mut grid = Grid::new(Input {});

    dbg!(&grid);

    dbg!(grid.generate(Place { x: 0, y: 0 }));

    dbg!(&grid);
}