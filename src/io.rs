use crate::game::{Game, Action, Direction::*};
use crate::helper::SizeUsize;
use crate::io::input::Input;
use std::{io, thread, sync::mpsc};
use clap::Parser;
use crossterm::event::KeyModifiers;
use crossterm::terminal::{self, disable_raw_mode};
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
        Show,
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
    pub fn new() -> Io {
        let input = Input::parse();
        let window_size = terminal::window_size().expect("failed to get terminal size");
        let window_size = SizeUsize {
            width:  window_size.columns as usize,
            height: window_size.rows    as usize,
        };
        Io {
            game: Game::new(
                input.mine_concentration,
                input.seed,
                window_size,
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

        buffer.execute(EnterAlternateScreen)?;
        buffer.execute(Hide)?;
        enable_raw_mode()?;
        loop {
            buffer.execute(Clear(All))?;

            if self.window_too_small() {
                buffer.execute(MoveTo(0, 0))?;
                buffer.execute(Print("window is too small"))?;
            } else {
                let view = self.game.view(self.window_size);
                view.render(&mut buffer)?;
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
                        self.game.action(Action::Resize(new_size));
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
        buffer.execute(Show)?;
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
}

