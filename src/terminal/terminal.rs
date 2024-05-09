use color_eyre::{eyre::WrapErr, Result};
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend;
use std::{io, panic};

use crate::event::TerminalEvent;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;
/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Terminal {
  /// Interface to the Terminal.
  terminal: CrosstermTerminal,

  /// A receiver channel to for terminal events.
  pub rx: tokio::sync::mpsc::UnboundedReceiver<TerminalEvent>,

  /// A transmission channel for terminal events.
  pub tx: tokio::sync::mpsc::UnboundedSender<TerminalEvent>,

  /// The terminal management task.
  pub task: tokio::task::JoinHandle<()>,

  /// How many times per second the tick event should be triggered.
  /// The default value is 4.
  pub tick_rate: f64,

  /// How many times per second the render event should be triggered.
  /// The default value is 60.
  pub frame_rate: f64,
}

impl Terminal {
  /// Constructs a new instance of [`Tui`].
  pub fn create() -> Result<Self> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let tx = tx.clone();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = ratatui::Terminal::new(backend)?;

    let task = tokio::spawn(async {});

    let tick_rate = 4.0;
    let frame_rate = 60.0;

    Ok(Self {
      terminal,
      rx,
      tx,
      task,
      tick_rate,
      frame_rate,
    })
  }

  /// Connects all of the terminal event handlers, and starts the terminal management task.
  pub fn start(&mut self) {
    let tick_rate = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
    let frame_rate = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);

    let tx = self.tx.clone();

    self.task = tokio::spawn(async move {
      let mut terminal_event_stream = crossterm::event::EventStream::new();
      let mut tick_interval = tokio::time::interval(tick_rate);
      let mut render_interval = tokio::time::interval(frame_rate);

      tx.send(TerminalEvent::Init).unwrap();

      loop {
        let tick_delay = tick_interval.tick();
        let render_delay = render_interval.tick();
        let crossterm_event = terminal_event_stream.next().fuse();

        tokio::select! {
          maybe_event = crossterm_event => {
            maybe_event
              .map(|event| event.wrap_err("failed to get terminal event"))
              .and_then(TerminalEvent::from_crossterm_event)
              .map(|event| tx.send(event).unwrap());
          },
          _ = tick_delay => {
              tx.send(TerminalEvent::Tick).unwrap();
          },
          _ = render_delay => {
              tx.send(TerminalEvent::Render).unwrap();
          },
        }
      }
    });
  }

  /// Stops the terminal management task, and unbinds everything.
  pub fn stop(&mut self) {
    let mut counter = 0;
    while !self.task.is_finished() {
      std::thread::sleep(std::time::Duration::from_millis(1));
      counter += 1;
      if counter > 50 {
        self.task.abort();
      }
      if counter > 100 {
        println!("Failed to abort task in 100 milliseconds for unknown reason");
        break;
      }
    }
    // self.task.abort();
    // self.tx.close();
    // self.rx.close();
  }

  /// Initializes the terminal interface.
  ///
  /// It enables the raw mode and sets terminal properties.
  pub fn enter(&mut self) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

    // Define a custom panic hook to reset the terminal properties.
    // This way, you won't have your terminal messed up if an unexpected error happens.
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
      Self::reset().expect("failed to reset the terminal");
      panic_hook(panic);
    }));

    self.terminal.hide_cursor()?;
    self.terminal.clear()?;
    self.start();

    Ok(())
  }

  /// Resets the terminal interface.
  ///
  /// This function is also used for the panic hook to revert
  /// the terminal properties if unexpected errors occur.
  fn reset() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
  }

  /// Exits the terminal interface.
  ///
  /// It disables the raw mode and reverts back the terminal properties.
  pub fn exit(&mut self) -> Result<()> {
    Self::reset()?;
    self.terminal.show_cursor()?;
    Ok(())
  }

  pub async fn next(&mut self) -> Result<TerminalEvent> {
    self
      .rx
      .recv()
      .await
      .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
  }
}

impl std::ops::Deref for Terminal {
  type Target = ratatui::Terminal<CrosstermBackend<std::io::Stderr>>;

  fn deref(&self) -> &Self::Target {
    &self.terminal
  }
}

impl std::ops::DerefMut for Terminal {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.terminal
  }
}

impl Drop for Terminal {
  fn drop(&mut self) {
    self.exit().unwrap();
  }
}
