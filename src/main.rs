pub mod helper;
pub mod grid;
pub mod view;
pub mod game;

fn main() -> std::io::Result<()> {
    game::start()
}
