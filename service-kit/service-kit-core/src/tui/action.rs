use crossterm::event::{KeyCode, KeyEventKind};

use crate::tui::event::TerminalEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Input(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Quit,
    Update,
}

impl TryFrom<TerminalEvent> for Action {
    type Error = ();

    fn try_from(event: TerminalEvent) -> Result<Self, Self::Error> {
        match event {
            TerminalEvent::Tick => Ok(Self::Update),
            TerminalEvent::Key(key_event) => {
                let code = key_event.code;
                let kind = key_event.kind;

                match (kind, code) {
                    (KeyEventKind::Press, KeyCode::Esc) => Ok(Self::Quit),
                    // How do I get to "a modifier key is pressed"?
                    // And then how do I get to "a modifier key is released"?
                    _ => Ok(Self::Input(key_event)),
                }
            }
            TerminalEvent::Mouse(mouse_event) => Ok(Self::Mouse(mouse_event)),
            _ => Err(()),
        }
    }
}
