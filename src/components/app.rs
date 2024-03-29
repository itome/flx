use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;

use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{Focus, State, Tab};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct AppComponent {}

impl AppComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for AppComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let Some(session) = CurrentSessionSelector.select(state) else {
            return;
        };

        let border_color = if state.current_focus == Focus::Tab(Tab::App) {
            Color::Green
        } else {
            Color::White
        };

        let device = state
            .devices
            .iter()
            .find(|d| d.id == session.device_id.clone().unwrap_or("".to_string()));
        let device_name = device.map(|d| d.name.clone()).unwrap_or("".to_string());
        let flavor = &session.flavor;
        let status_color = if session.hot_reloading {
            Color::Yellow
        } else if session.hot_restarting {
            Color::LightMagenta
        } else if !session.started {
            Color::DarkGray
        } else {
            Color::White
        };
        let mut name = format!(" {} ", device_name);
        if let Some(flavor) = flavor {
            name.push_str(&format!("({})", flavor))
        }

        let block = Block::default()
            .title("App")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));
        let text = Paragraph::new(name)
            .style(Style::default().fg(status_color))
            .block(block);
        f.render_widget(text, area);
    }
}
