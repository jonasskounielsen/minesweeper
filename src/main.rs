pub mod grid;
pub mod view;
pub mod game;
pub mod helper;

use game::Game;
use grid::Grid;
use helper::SizeUsize;

fn main() {
    let /* mut */ grid = Grid::new(0.3f32, 0xDEADBEEF);
    let game = Game::new(grid);

    let view = game.view(SizeUsize { width: 16, height: 16 });

    println!("{}", view.render());
}