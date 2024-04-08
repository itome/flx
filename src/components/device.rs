use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use redux_rs::StoreApi;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use crate::{
    redux::{
        action::Action,
        state::{Focus, Home, State},
        ActionOrThunk,
    },
    tui::Frame,
};
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

pub struct DeviceComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl Default for DeviceComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceComponent {
    pub fn new() -> Self {
        Self { action_tx: None }
    }
}

impl Component for DeviceComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let device = if let Some(device_id) = &state.selected_device_id {
            state.devices.iter().find(|d| d.id == *device_id)
        } else {
            None
        };

        let Some(device) = device else {
            f.render_widget(Paragraph::new("No device selected").block(block), area);
            return;
        };

        let widths = [Constraint::Length(16), Constraint::Fill(1)];
        let table = Table::new(
            vec![
                Row::new([
                    Cell::from("ID").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.id.clone()),
                ]),
                Row::new([
                    Cell::from("Name").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.name.clone()),
                ]),
                Row::new([
                    Cell::from("Platform").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.platform.clone()),
                ]),
                Row::new([
                    Cell::from("Category").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.category.clone()),
                ]),
                Row::new([
                    Cell::from("Is emulator").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.emulator.to_string()),
                ]),
                Row::new([
                    Cell::from("Is ephemeral").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.ephemeral.to_string()),
                ]),
                Row::new([
                    Cell::from("SDK").style(Style::default().fg(Color::Yellow).bold()),
                    Cell::from(device.sdk.to_string()),
                ]),
            ],
            widths,
        )
        .block(block);
        f.render_widget(table, area);
    }
}
