use crate::action::Action;
use color_eyre::Result;
use ratatui::{
  layout::{Constraint, Direction, Layout},
  widgets::{Block, Borders},
};
use tui_textarea::TextArea;

use super::{ActionContext, Component};

const MIN_HEIGHT: usize = 2;

#[derive(Clone, Debug, Default)]
pub struct Input<'a> {
  pub textarea: TextArea<'a>,
}

impl<'a> Input<'a> {
  /// Create a new text area input
  pub fn new() -> Self {
    let mut textarea = TextArea::default();
    textarea.set_block(
      Block::default()
        .borders(Borders::ALL)
        .title("Textarea with Variable Height"),
    );

    Self { textarea }
  }

  fn last_input(&mut self, input: crossterm::event::KeyEvent) {
    self.textarea.input(input);
  }

  pub fn size(&self) -> u16 {
    // + 2 for borders
    std::cmp::max(self.textarea.lines().len(), MIN_HEIGHT) as u16 + 2
  }
}

impl<'a> Component for Input<'a> {
  fn update(&mut self, context: ActionContext) -> Result<Option<Action>> {
    match context.action {
      Action::Input(input) => {
        self.last_input(input);
      }
      Action::Quit => {}
      _ => {}
    }
    Ok(None)
  }

  fn view(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(self.size()), Constraint::Min(0)].as_slice())
      .split(area);

    frame.render_widget(self.textarea.widget(), chunks[0]);
  }
}
