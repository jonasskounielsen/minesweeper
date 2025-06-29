use crate::game::{Game, Action, Direction::*};
use crate::helper::{SizeI32, SizeUsize};
use crate::io::input::Input;
use crate::view::View;
use std::{io, thread, sync::mpsc};
use clap::Parser;
use crossterm::event::KeyModifiers;
use crossterm::terminal::{self, disable_raw_mode, WindowSize};
use crossterm::{
    event::{
        self,
        read,
        KeyEvent,
        KeyCode,
        Event as TerminalEvent,
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
        Hide,
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
    pub const CURSOR_PADDING: SizeI32 = SizeI32 {
        width:  3,
        height: 3,
    };

    pub fn new() -> Io {
        let input = Input::parse();
        let window_size = terminal::window_size().expect("failed to get terminal size");
        let window_size = SizeUsize {
            width:  window_size.columns as usize,
            height: window_size.rows    as usize,
        };
        let max_cursor_displacement = Self::max_cursor_displacement(window_size);
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
                tx_key.send(IoEvent::CrosstermEvent(read()?)).expect("failed to send io event to main thread");
            }
        });

        let tx_time = tx;
        thread::spawn(move || {
            loop {
                thread::sleep(Game::time_until_timer_update());
                tx_time.send(IoEvent::Second).expect("failed to send io event to main thread");
            }
        });

        // buffer.execute(EnterAlternateScreen)?;
        buffer.execute(Hide)?;
        enable_raw_mode()?;
        loop {
            buffer.execute(Clear(All))?;

            if self.window_too_small() {
                buffer.execute(MoveTo(0, 0))?;
                buffer.execute(Print("window is too small"))?;
            } else {
                let view = self.game.view(self.window_size);

                for (i, line) in view.render().iter().enumerate() {
                    buffer.execute(MoveTo(0, i as u16))?;
                    buffer.execute(Print(line))?;
                }
            }
            

            match rx.recv().expect("failed to receive Io event") {
                IoEvent::CrosstermEvent(event) => {
                    match event {
                    TerminalEvent::Key(KeyEvent {
                        code: KeyCode::Char('c'), modifiers, ..
                    }) if modifiers.contains(KeyModifiers::CONTROL) => {
                        Self::quit(buffer)?;
                        return Ok(());
                    },
                    TerminalEvent::Key(key_event) if !self.window_too_small() => {
                        self.parse_key(key_event)
                    },
                    TerminalEvent::Resize(new_width, new_height) => {
                        let new_size = SizeUsize {
                            width:  new_width  as usize,
                            height: new_height as usize,
                        };
                        self.window_size = new_size;
                        let new_max_cursor_displacement = Self::max_cursor_displacement(new_size);
                        self.game.max_cursor_displacement = new_max_cursor_displacement;
                    },
                    _ => (),
                }
                },
                IoEvent::Second => (), // update display when timer increments
            }
        }
    }

    fn quit(mut buffer: impl io::Write) -> io::Result<()> {
        buffer.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn parse_key(&mut self, key: KeyEvent) {
        if key.modifiers != event::KeyModifiers::NONE ||
           key.kind != event::KeyEventKind::Press {
            return;
        }
        let action = match key.code {
            KeyCode::Left      => Action::MoveCursor(Left),
            KeyCode::Right     => Action::MoveCursor(Right),
            KeyCode::Down      => Action::MoveCursor(Down),
            KeyCode::Up        => Action::MoveCursor(Up),

            KeyCode::Char(' ') => Action::Reveal,
            KeyCode::Char('a') => Action::RevealAdjacent,
            KeyCode::Char('f') => Action::Flag,

            KeyCode::Char('r') => Action::Reset,
            _ => return,
        };
        self.game.action(action);
    }

    fn window_too_small(&self) -> bool {
        self.game.window_too_small(self.window_size)
    }

    fn max_cursor_displacement(window_size: SizeUsize) -> SizeI32 {
        let matrix_size = View::matrix_size(window_size);
        SizeI32 {
            width:  matrix_size.width  as i32 - Self::CURSOR_PADDING.width  * 2,
            height: matrix_size.height as i32 - Self::CURSOR_PADDING.height * 2,
        }
    }
}

