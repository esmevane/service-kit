use crate::action::Action;
use bevy::app::App;
use color_eyre::Result;
use ratatui::layout::Rect;

use super::{ActionContext, Component};

/// Our game component wraps around a Bevy ECS implementation, hands our
/// terminal events to that ECS, updates it once we receive an event, and
/// turns it into a view we can render in our terminal.
///
#[derive(Debug)]
pub struct Game {
  /// This is not the program app, but instead an instance of Bevy's App
  /// struct, that allows us to tap into its ECS power.
  pub game_app: App,
}

impl Game {
  /// Create a new application
  pub fn new() -> Self {
    Self {
      game_app: App::new(),
    }
  }
}

impl Component for Game {
  fn update(&mut self, context: ActionContext) -> Result<Option<Action>> {
    match context.action {
      _ => {}
    }

    Ok(None)
  }

  fn view(&self, _frame: &mut ratatui::Frame, _area: Rect) {}
}
