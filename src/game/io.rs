use crate::game::input::Keybinds;
use crate::game::{Game, Action, Direction::*};
use crate::helper::SizeUsize;
use std::sync::mpsc::{Receiver, Sender};
use std::{io, thread, sync::mpsc};
use crossterm::event::KeyModifiers;
use crossterm::terminal::{disable_raw_mode, Clear, ClearType};
use crossterm::{
    event::{
        self,
        read,
        KeyEvent,
        KeyCode,
        Event as TerminalEvent,
    },
    terminal::{
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    cursor::{
        Show,
        Hide,
    },
    ExecutableCommand,
};

pub enum IoEvent {
    CrosstermEvent(crossterm::event::Event),
    Second,
    Panic(&'static str),
}

#[derive(Debug)]
pub struct Io<'a> {
    game:        &'a mut Game,
    window_size: SizeUsize,
    keybinds:    Keybinds,
    rx:          Receiver<IoEvent>,
    tx:          Sender<IoEvent>,
}

impl<'a> Io<'a> {
    pub fn new(game: &mut Game, window_size: SizeUsize, keybinds: Keybinds) -> Io<'_> {
        let (tx, rx) = mpsc::channel();
        game.tx_panic = Some(tx.clone());
        Io { game, window_size, keybinds, rx, tx }
    }

    pub fn run(&mut self, mut buffer: impl io::Write) -> io::Result<()> {
        let tx_key = self.tx.clone();
        thread::spawn(move || -> io::Result<()> {
            loop {
                tx_key.send(IoEvent::CrosstermEvent(read()?)).expect("failed to send io event to main thread");
            }
        });

        let tx_time = self.tx.clone();
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
            let view = self.game.view();
            view.render(&mut buffer)?;
            buffer.flush()?;
            self.handle_event(&mut buffer, self.rx.recv().expect("failed to receive io event"))?;
        }
    }

    fn handle_event(&mut self, buffer: &mut impl std::io::Write, event: IoEvent) -> io::Result<()> {
        match event {
            IoEvent::CrosstermEvent(event) => {
                match event {
                    TerminalEvent::Key(KeyEvent {
                        code: KeyCode::Char('c'), modifiers, ..
                    }) if modifiers.contains(KeyModifiers::CONTROL) => {
                        Self::quit(buffer)?;
                        return Ok(());
                    },
                    TerminalEvent::Key(key_event) => {
                        self.parse_key(key_event)
                    },
                    TerminalEvent::Resize(new_width, new_height) => {
                        self.resize(buffer, new_width, new_height)?
                    },
                    _ => (),
                }
            },
            IoEvent::Second => (), // update display when timer increments
            IoEvent::Panic(message) => {
                Self::quit(buffer)?;
                eprintln!("{}", message);
                return Ok(());
            },
        }
        Ok(())
    }

    fn resize(&mut self, buffer: &mut impl io::Write, new_width: u16, new_height: u16) -> io::Result<()> {
        let new_size = SizeUsize {
            width:  new_width  as usize,
            height: new_height as usize,
        };
        self.window_size = new_size;
        self.game.action(Action::Resize(new_size));
        buffer.execute(Clear(ClearType::All))?;
        Ok(())
    }

    fn quit(buffer: &mut impl io::Write) -> io::Result<()> {
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
        if let Some(action) = self.keybinds.action(key.code) {
            self.game.action(action);
        }
    }
}

impl Keybinds {
    pub fn action(&self, key_code: KeyCode) -> Option<Action> {
        match self {
            Self::Vim    => self.action_vim   (key_code),
            Self::Wasd   => self.action_wasd  (key_code),
            Self::Arrows => self.action_arrows(key_code),
        }
    }

    fn action_vim(&self, key_code: KeyCode) -> Option<Action> {
        match key_code {
            KeyCode::Char('h') => Some(Action::MoveCursor(Left)),
            KeyCode::Char('j') => Some(Action::MoveCursor(Down)),
            KeyCode::Char('k') => Some(Action::MoveCursor(Up)),
            KeyCode::Char('l') => Some(Action::MoveCursor(Right)),

            KeyCode::Char(' ') => Some(Action::Reveal),
            KeyCode::Char('d') => Some(Action::RevealAdjacent),
            KeyCode::Char('f') => Some(Action::Flag),

            KeyCode::Char('r') => Some(Action::Reset),
            _ => None,
        }
    }

    fn action_wasd(&self, key_code: KeyCode) -> Option<Action> {
        match key_code {
            KeyCode::Char('w') => Some(Action::MoveCursor(Up)),
            KeyCode::Char('a') => Some(Action::MoveCursor(Left)),
            KeyCode::Char('s') => Some(Action::MoveCursor(Down)),
            KeyCode::Char('d') => Some(Action::MoveCursor(Right)),

            KeyCode::Char(' ') => Some(Action::Reveal),
            KeyCode::Char('j') => Some(Action::RevealAdjacent),
            KeyCode::Char('k') => Some(Action::Flag),

            KeyCode::Char('r') => Some(Action::Reset),
            _ => None,
        }
    }

    fn action_arrows(&self, key_code: KeyCode) -> Option<Action> {
        match key_code {
            KeyCode::Left      => Some(Action::MoveCursor(Left)),
            KeyCode::Right     => Some(Action::MoveCursor(Right)),
            KeyCode::Down      => Some(Action::MoveCursor(Down)),
            KeyCode::Up        => Some(Action::MoveCursor(Up)),

            KeyCode::Char(' ') => Some(Action::Reveal),
            KeyCode::Char('d') => Some(Action::RevealAdjacent),
            KeyCode::Char('f') => Some(Action::Flag),

            KeyCode::Char('r') => Some(Action::Reset),
            _ => None,
        }
    }
}

