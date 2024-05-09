use crate::{
  action::Action,
  components::{window::Window, ActionContext},
};

/// Application state
#[derive(Debug)]
pub struct App {
  /// Should the application quit?
  pub should_quit: bool,
  /// A sender channel to send async actions to the application.
  pub action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
  /// Components
  pub components: Vec<Box<dyn crate::components::Component>>,
}

impl App {
  /// Create a new application
  pub fn new(action_tx: tokio::sync::mpsc::UnboundedSender<Action>) -> Self {
    App {
      should_quit: false,
      action_tx,
      components: vec![Box::new(Window::new())],
    }
  }

  /// Update a running application
  pub fn update(&mut self, action: Action) -> Option<Action> {
    match action {
      Action::Quit => self.quit(),
      _ => {}
    }

    // loop through all components and update them with the action context
    for component in self.components.iter_mut() {
      let next_action = component.update(ActionContext {
        action_tx: self.action_tx.clone(),
        config: crate::config::Config::default(),
        action: action.clone(),
      });

      if let Ok(Some(action)) = next_action {
        return Some(action);
      }
    }

    None
  }

  /// Render the application, given a frame, by looping through all components
  /// and rendering them.
  pub fn render(&self, frame: &mut ratatui::Frame) {
    for component in &self.components {
      component.view(frame, frame.size());
    }
  }

  /// Quits the application
  pub fn quit(&mut self) {
    self.should_quit = true;
  }
}
