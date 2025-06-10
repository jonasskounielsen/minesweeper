use crate::game::Game;
use crate::grid::Grid;
use crate::helper::SizeUsize;
use crate::io::input::Input;
use clap::Parser;

mod input; 

#[derive(Debug)]
pub struct Io {
    game: Game,
    viewSize: SizeUsize,
}

impl Io {
    pub fn new() -> Io {
        let input = Input::parse();
        Io {
            game: Game::new(Grid::new(input.mine_concentration, input.seed)),
            viewSize: SizeUsize { width: input.width, height: input.height },
        }
    }
}

