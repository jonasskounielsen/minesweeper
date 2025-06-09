pub mod grid;
pub mod view;
pub mod game;

use game::Game;
use grid::{Grid, Place};
use view::Size;

fn main() {
    let /* mut */ grid = Grid::new(0.3f32, 0xDEADBEEF);
    let game = Game::new(grid);

    let view = game.view(Size { width: 16, height: 16 });

    println!("{}", view.render());
}