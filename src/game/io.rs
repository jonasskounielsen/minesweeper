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
    game: &'a mut Game,
    window_size: SizeUsize,
    rx: Receiver<IoEvent>,
    tx:   Sender<IoEvent>,
}

impl<'a> Io<'a> {
    pub fn new(game: &mut Game, window_size: SizeUsize) -> Io {
        let (tx, rx) = mpsc::channel();
        game.tx_panic = Some(tx.clone());
        Io { game, window_size, rx, tx }
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
            match self.rx.recv().expect("failed to receive io event") {
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
                            let new_size = SizeUsize {
                                width:  new_width  as usize,
                                height: new_height as usize,
                            };
                            self.window_size = new_size;
                            self.game.action(Action::Resize(new_size));
                            buffer.execute(Clear(ClearType::All))?;
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
}

