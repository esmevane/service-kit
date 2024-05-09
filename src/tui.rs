mod action;
mod app;
mod components;
mod config;
mod event;
mod terminal;

use app::App;
use color_eyre::Result;
use event::TerminalEvent;
use terminal::Terminal;

pub async fn init() -> Result<()> {
    // Create an application.
    let (action_tx, mut action_rx) = tokio::sync::mpsc::unbounded_channel();
    let mut app = App::new(action_tx.clone());

    let mut terminal = Terminal::create()?;

    terminal.enter()?;

    // Start the main loop.
    while !app.should_quit {
        let event = terminal.next().await?;

        // Render if requested.
        if let TerminalEvent::Render = event {
            terminal.draw(|frame| app.render(frame))?;
        }

        if let Ok(action) = event.try_into() {
            action_tx.send(action).unwrap();
        }

        // Update the application state.
        if let Ok(action) = action_rx.try_recv() {
            app.update(action);
        }
    }

    // Exit the user interface.
    terminal.exit()?;

    Ok(())
}
