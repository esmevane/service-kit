use color_eyre::{eyre::Error, Result};
use crossterm::event::Event as CrosstermEvent;

#[derive(Debug)]
pub enum TerminalEvent {
    Init,
    Error(Error),
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(crossterm::event::KeyEvent),
    Release(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
}

impl TerminalEvent {
    pub fn from_crossterm_event(event: Result<CrosstermEvent>) -> Option<Self> {
        match event {
            Ok(given_event) => match given_event {
                CrosstermEvent::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        Some(TerminalEvent::Key(key))
                    } else if key.kind == crossterm::event::KeyEventKind::Release {
                        Some(TerminalEvent::Release(key))
                    } else {
                        None
                    }
                }
                CrosstermEvent::Mouse(mouse) => Some(TerminalEvent::Mouse(mouse)),
                CrosstermEvent::Resize(x, y) => Some(TerminalEvent::Resize(x, y)),
                CrosstermEvent::FocusLost => Some(TerminalEvent::FocusLost),
                CrosstermEvent::FocusGained => Some(TerminalEvent::FocusGained),
                CrosstermEvent::Paste(string) => Some(TerminalEvent::Paste(string)),
            },
            Err(error) => Some(TerminalEvent::Error(error)),
        }
    }
}
