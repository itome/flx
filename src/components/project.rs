use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{Focus, Home, State};
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
        let border_color = if state.focus == Focus::Home(Home::Project) && state.popup.is_none() {
            Color::Green
        } else {
            Color::White
        };

        let block = Block::default()
            .title("Project")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));
        let text = Paragraph::new("flx").block(block);
        f.render_widget(text, area);
    }
}
