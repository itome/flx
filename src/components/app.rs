use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;

use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{DevTools, Focus, State};
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
        let Some(session) = current_session_selector(state) else {
            return;
        };

        let border_color = if state.focus == Focus::DevTools(DevTools::App) && state.popup.is_none()
        {
            Color::Green
        } else {
            Color::White
        };

        let device = if let Some(device_id) = &session.device_id {
            state.devices.iter().find(|d| &d.id == device_id)
        } else {
            None
        };
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
        let mut name = format!(
            " {} ",
            if let Some(device) = device {
                device.name.clone()
            } else {
                "".to_string()
            }
        );
        if let Some(flavor) = flavor {
            name.push_str(&format!("({})", flavor))
        }

        let block = Block::default()
            .title("App")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));
        let text = Paragraph::new(name)
            .style(Style::default().fg(status_color))
            .block(block);
        f.render_widget(text, area);
    }
}
