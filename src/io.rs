use crate::game::{Game, Action, Direction::*};
use crate::helper::{SizeI32, SizeUsize};
use crate::io::input::Input;
use std::time::Duration;
use std::{io, thread, sync::mpsc};
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
    cursor::{
        MoveTo,
    },
    ExecutableCommand,
};

mod input;

enum Event {
    Key(KeyEvent),
    Second,
}

#[derive(Debug)]
pub struct Io {
    game: Game,
    view_size: SizeUsize,
}

impl Io {
    pub fn new() -> Io {
        let input = Input::parse();
        let max_cursor_displacement = SizeI32 {
            width:  input.width  as i32 - 6,
            height: input.height as i32 - 6,
        };
        Io {
            game: Game::new(
                input.mine_concentration,
                input.seed,
                max_cursor_displacement,
            ),
            view_size: SizeUsize { width: input.width, height: input.height },
        }
    }

    pub fn run(&mut self, mut buffer: impl io::Write) -> io::Result<()> {
        let (tx, rx) = mpsc::channel();

        let tx_key = tx.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                if let event::Event::Key(key_event) = read()? {
                    tx_key.send(Event::Key(key_event)).unwrap();
                }
            }
        });

        let tx_time = tx;
        let initial_wait = self.game.time_until_timer_update();
        thread::spawn(move || {
            thread::sleep(initial_wait);
            loop {
                thread::sleep(Duration::from_secs(1));
                tx_time.send(Event::Second).unwrap();
            }
        });

        //buffer.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        loop {
            buffer.execute(Clear(All))?;

            let view = self.game.view(self.view_size);

            for (i, line) in view.render().iter().enumerate() {
                buffer.execute(MoveTo(0, i as u16))?;
                buffer.execute(Print(line))?;
            }
            

            match rx.recv().unwrap() {
                Event::Key(key_event) => self.parse_key(key_event),
                Event::Second => (), // update display when timer increments
            }
        }
    }

    fn parse_key(&mut self, key: KeyEvent) {
        if key.modifiers != event::KeyModifiers::NONE ||
           key.kind != event::KeyEventKind::Press {

        }
        let action = match key.code {
            KeyCode::Left      => Action::MoveCursor(Left),
            KeyCode::Right     => Action::MoveCursor(Right),
            KeyCode::Down      => Action::MoveCursor(Down),
            KeyCode::Up        => Action::MoveCursor(Up),

            KeyCode::Enter     => Action::Reveal,
            KeyCode::Backspace => Action::RevealAdjacent,
            KeyCode::Char(' ') => Action::Flag,

            KeyCode::Char('r') => Action::Reset,
            _ => return,
        };
        self.game.action(action);
    }
}

