use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;

use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{DevTools, Focus, Home, State};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct NetworkComponent {}

impl NetworkComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for NetworkComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color =
            if state.focus == Focus::DevTools(DevTools::Network) && state.popup.is_none() {
                Color::Green
            } else {
                Color::White
            };
        let block = Block::default()
            .title("Network")
            .padding(Padding::horizontal(1))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .borders(Borders::ALL);

        f.render_widget(block, area);
    }
}
