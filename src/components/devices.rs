use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};
use redux_rs::{middlewares::thunk::ActionOrThunk, StoreApi};
use tokio::sync::Mutex;

use crate::{
    redux::state::{Focus, State, Tab},
    tui::Frame,
};
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

pub struct DevicesComponent {}

impl Default for DevicesComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl DevicesComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for DevicesComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let devices = state.devices.clone();
        let border_color = if state.current_focus == Focus::Tab(Tab::Devices) {
            Color::Green
        } else {
            Color::White
        };

        let block = Block::default()
            .title("Devices")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let items: Vec<ListItem> = devices
            .iter()
            .map(|d| ListItem::new(format!("{} ({})", d.name, d.platform)))
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .block(block);

        f.render_widget(list, area);
    }
}
