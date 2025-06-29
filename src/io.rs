use crate::game::{Game, Action, Direction::*};
use crate::helper::{SizeI32, SizeUsize};
use crate::io::input::Input;
use crate::view::View;
use std::time::{self, Duration};
use std::{io, thread, sync::mpsc};
use clap::Parser;
use crossterm::terminal;
use crossterm::{
    event::{
        self,
        read,
        KeyEvent,
        KeyCode,
        Event,
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

enum IoEvent {
    CrosstermEvent(crossterm::event::Event),
    Second,
}

#[derive(Debug)]
pub struct Io {
    game: Game,
    window_size: SizeUsize,
}

impl Io {
    pub fn new() -> Io {
        let input = Input::parse();
        let window_size = terminal::window_size().expect("failed to get terminal size");
        let window_size = SizeUsize {
            width:  window_size.columns as usize,
            height: window_size.rows    as usize,
        };
        let matrix_size = View::matrix_size(window_size);
        let max_cursor_displacement = SizeI32 {
            width:  matrix_size.width  as i32 - 6,
            height: matrix_size.height as i32 - 6,
        };
        Io {
            game: Game::new(
                input.mine_concentration,
                input.seed,
                max_cursor_displacement,
            ),
            window_size,
        }
    }

    pub fn run(&mut self, mut buffer: impl io::Write) -> io::Result<()> {
        let (tx, rx) = mpsc::channel();

        let tx_key = tx.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                tx_key.send(IoEvent::CrosstermEvent(read()?)).expect("failed to send Io event");
            }
        });

        let tx_time = tx;
        thread::spawn(move || {
            loop {
                thread::sleep(Game::time_until_timer_update());
                tx_time.send(IoEvent::Second).expect("failed to send Io event");
            }
        });

        //buffer.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        loop {
            buffer.execute(Clear(All))?;

            let view = self.game.view(self.window_size);

            for (i, line) in view.render().iter().enumerate() {
                buffer.execute(MoveTo(0, i as u16))?;
                buffer.execute(Print(line))?;
            }
            

            match rx.recv().expect("failed to receive Io event") {
                IoEvent::CrosstermEvent(event) => self.parse_event(event),
                IoEvent::Second => (), // update display when timer increments
            }
        }
    }

    fn parse_event(&mut self, event: event::Event) {
        match event {
            Event::Key(key_event)  => self.parse_key   (key_event),
            Event::Resize(new_width, new_height) => {
                self.window_size = SizeUsize {
                    width:  new_width  as usize,
                    height: new_height as usize,
                }
            },
            _ => (),
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

