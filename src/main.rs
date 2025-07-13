pub mod helper;
pub mod grid;
pub mod view;
pub mod game;

use game::Game;

fn main() -> std::io::Result<()> {
    Game::start()
}