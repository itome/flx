use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{State, Tab};
use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

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
        let default_color = if state.selected_tab == Tab::Project {
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
