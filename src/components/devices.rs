use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};
use redux_rs::{middlewares::thunk::ActionOrThunk, StoreApi};
use tokio::sync::Mutex;

use crate::{
    daemon::{flutter::FlutterDaemon, io::device::Device},
    redux::state::State,
    tui::Frame,
};
use color_eyre::eyre::Result;

use super::Component;

pub struct DevicesComponent {}

impl DevicesComponent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for DevicesComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) -> Result<()> {
        let devices = state.devices.clone();
        let default_color = Color::White;

        let block = Block::default()
            .title("Devices")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        let items: Vec<ListItem> = devices
            .iter()
            .map(|d| {
                ListItem::new(format!("{} ({})", d.name, d.platform))
                    .style(Style::default().fg(default_color))
            })
            .collect();
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .block(block);

        f.render_widget(list, area);
        Ok(())
    }
}
