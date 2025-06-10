use crate::game::{Game, Action, Direction::*};
use crate::grid::Grid;
use crate::helper::SizeUsize;
use crate::io::input::Input;
use std::io;
use clap::Parser;
use crossterm::{
    event::{
        self,
        read,
        KeyEvent,
        KeyCode,
    },
    style::Print,
    terminal::{
        enable_raw_mode,
        Clear,
        ClearType::All,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

mod input; 

#[derive(Debug)]
pub struct Io {
    game: Game,
    view_size: SizeUsize,
}

impl Io {
    pub fn new() -> Io {
        let input = Input::parse();
        Io {
            game: Game::new(Grid::new(input.mine_concentration, input.seed)),
            view_size: SizeUsize { width: input.width, height: input.height },
        }
    }

    pub fn run(&mut self, mut buffer: impl io::Write) -> io::Result<()> {
        buffer.execute(EnterAlternateScreen)?;
        enable_raw_mode();
        loop {
            buffer.execute(Clear(All))?;

            let view = self.game.view(self.view_size);
            buffer.execute(Print(view.render()))?;

            match read()? {
                crossterm::event::Event::Key(key_event) => {
                    if key_event.modifiers != event::KeyModifiers::NONE &&
                       key_event.kind != event::KeyEventKind::Press {
                        self.parse_key(key_event);
                    }
                },
                _ => (),
            }
        }
    }

    fn parse_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left      => self.game.action(Action::MoveCursor(Left)),
            KeyCode::Right     => self.game.action(Action::MoveCursor(Right)),
            KeyCode::Down      => self.game.action(Action::MoveCursor(Down)),
            KeyCode::Up        => self.game.action(Action::MoveCursor(Up)),

            KeyCode::Enter     => self.game.action(Action::Reveal),
            KeyCode::Char(' ') => self.game.action(Action::Flag),
            _ => (),
        }
    }
}

