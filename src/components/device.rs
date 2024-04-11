use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use redux_rs::StoreApi;
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use crate::{
    redux::{
        action::Action,
        selector::device_or_emulators::{self, device_or_emulators_selector, DeviceOrEmulator},
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

        let device_or_emulators = device_or_emulators_selector(state);
        let device_or_emulator = if let Some(device_id) = &state.selected_device_or_emulator_id {
            device_or_emulators.iter().find(|d| match d {
                DeviceOrEmulator::Device(device) => &device.id == device_id,
                DeviceOrEmulator::Emulator(emulator) => &emulator.id == device_id,
            })
        } else {
            None
        };

        let Some(device_or_emulator) = device_or_emulator else {
            f.render_widget(Paragraph::new("No device selected").block(block), area);
            return;
        };

        let widths = [Constraint::Length(16), Constraint::Fill(1)];
        let table = Table::new(
            match device_or_emulator {
                DeviceOrEmulator::Device(device) => {
                    let mut rows = vec![
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
                            Cell::from("Is emulator")
                                .style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(device.emulator.to_string()),
                        ]),
                        Row::new([
                            Cell::from("Is ephemeral")
                                .style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(device.ephemeral.to_string()),
                        ]),
                        Row::new([
                            Cell::from("SDK").style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(device.sdk.to_string()),
                        ]),
                    ];
                    let flavors = &state.flavors.get(&device.platform);
                    if let Some(flavors) = flavors {
                        if !flavors.is_empty() {
                            rows.push(Row::new([
                                Cell::from("Flavors")
                                    .style(Style::default().fg(Color::Yellow).bold()),
                                Cell::from(flavors.join(", ")),
                            ]));
                        }
                    }
                    rows
                }
                DeviceOrEmulator::Emulator(emulator) => {
                    let mut rows = vec![
                        Row::new([
                            Cell::from("ID").style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(emulator.id.clone()),
                        ]),
                        Row::new([
                            Cell::from("Name").style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(emulator.name.clone()),
                        ]),
                        Row::new([
                            Cell::from("Platform").style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(emulator.platform_type.clone()),
                        ]),
                        Row::new([
                            Cell::from("Category").style(Style::default().fg(Color::Yellow).bold()),
                            Cell::from(emulator.category.clone()),
                        ]),
                    ];
                    let flavors = &state.flavors.get(&emulator.platform_type);
                    if let Some(flavors) = flavors {
                        if !flavors.is_empty() {
                            rows.push(Row::new([
                                Cell::from("Flavors")
                                    .style(Style::default().fg(Color::Yellow).bold()),
                                Cell::from(flavors.join(", ")),
                            ]));
                        }
                    }
                    rows
                }
            },
            widths,
        )
        .block(block);
        f.render_widget(table, area);
    }
}
