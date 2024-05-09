use crate::tui::action::Action;
use color_eyre::Result;
use ratatui::layout::{Constraint, Direction, Layout};

use super::{display::Display, input::Input, ActionContext, Component};

#[derive(Clone, Debug, Default)]
pub struct Window<'a> {
    pub textarea: Input<'a>,
    pub display: Display,
}

impl<'a> Window<'a> {
    /// Create a new text area input
    pub fn new() -> Self {
        Self {
            textarea: Input::new(),
            display: Display::new(),
        }
    }
}

impl<'a> Component for Window<'a> {
    fn update(&mut self, context: ActionContext) -> Result<Option<Action>> {
        let text = self.textarea.update(context.clone())?;
        let display = self.display.update(context.clone())?;

        Ok(text.or(display))
    }

    fn view(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(self.textarea.size())].as_slice())
            .split(area);

        self.display.view(frame, layout[0]);
        self.textarea.view(frame, layout[1]);
    }
}
