pub mod helper;
pub mod grid;
pub mod view;
pub mod game;
pub mod io;

use io::Io;

fn main() -> std::io::Result<()> {
    let mut io = Io::new();
    io.run(std::io::stdout())
}