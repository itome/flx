use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{Focus, State, Tab};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct ProjectComponent {}

impl ProjectComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for ProjectComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let default_color = if state.current_focus == Focus::Tab(Tab::Project) {
            Color::White
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Project")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));
        let text = Paragraph::new("flx")
            .style(Style::default().fg(default_color))
            .block(block);
        f.render_widget(text, area);
    }
}
