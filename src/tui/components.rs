/// A full-screen counter.
pub mod display;

/// A text area at the bottom part of the console.
pub mod input;

/// The window which manages the display and the input.
pub mod window;

use crate::tui::{action::Action, config::Config};
use color_eyre::Result;
use ratatui::layout::Rect;

// let tx = context.action_tx.clone();
// tokio::spawn(async move {
//   tokio::time::sleep(std::time::Duration::from_secs(5)).await; // simulate network request
//   tx.send(Action::Increment).unwrap();
// });
#[derive(Clone, Debug)]
pub struct ActionContext {
    pub action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    pub config: Config,
    pub action: Action,
}

pub trait Component: std::fmt::Debug {
    fn view(&self, frame: &mut ratatui::Frame, area: Rect);
    fn update(&mut self, action: ActionContext) -> Result<Option<Action>>;
}
