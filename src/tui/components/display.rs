use crate::tui::action::Action;
use color_eyre::Result;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::{ActionContext, Component};

/// Text display for terminal UI
#[derive(Clone, Debug, Default)]
pub struct Display {
    pub text: Vec<String>,
}

impl Display {
    /// Create a new display
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Display {
    fn update(&mut self, context: ActionContext) -> Result<Option<Action>> {
        self.text.push(format!("{:?}", context.action));

        Ok(None)
    }

    fn view(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let height = area.height as usize;
        let last_actions: Vec<String> = self
            .text
            .iter()
            .rev()
            .take(height - 2)
            .rev()
            .cloned()
            .collect();
        let mut lines = vec![];

        for line in last_actions {
            lines.push(Line::from(line.clone()).alignment(Alignment::Left));
        }

        frame.render_widget(
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .title("Console app")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            area,
        )
    }
}
