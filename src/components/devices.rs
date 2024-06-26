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
        thunk::ThunkAction,
        ActionOrThunk,
    },
    tui::Frame,
};
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct DevicesComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
    state: ListState,
}

impl DevicesComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextDevice.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousDevice.into())?;
        Ok(())
    }

    fn launch_emulator(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::LaunchEmulator.into())?;
        Ok(())
    }
}

impl Component for DevicesComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::Home(Home::Devices) || state.popup.is_some() {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            KeyCode::Enter => self.launch_emulator()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color = if state.focus == Focus::Home(Home::Devices) && state.popup.is_none() {
            Color::Green
        } else {
            Color::White
        };

        let device_or_emulators = device_or_emulators_selector(state);

        let selected_index = if let Some(selected_device_id) = &state.selected_device_or_emulator_id
        {
            device_or_emulators
                .iter()
                .position(|device_or_emulator| match device_or_emulator {
                    DeviceOrEmulator::Device(device) => &device.id == selected_device_id,
                    DeviceOrEmulator::Emulator(emulator) => &emulator.id == selected_device_id,
                })
        } else {
            None
        };
        self.state.select(selected_index);

        let block = Block::default()
            .title("Devices")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let items: Vec<ListItem> = device_or_emulators
            .iter()
            .map(|device_or_emulator| match device_or_emulator {
                DeviceOrEmulator::Device(device) => ListItem::new(device.name.clone()),
                DeviceOrEmulator::Emulator(emulator) => {
                    ListItem::new(format!("▶ Start {}", emulator.name.clone()))
                }
            })
            .collect();

        let mut scrollbar_state =
            ScrollbarState::new(items.len()).position(selected_index.unwrap_or(0));
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(if state.focus == Focus::Home(Home::Devices) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            })
            .highlight_spacing(HighlightSpacing::Never)
            .block(block);

        f.render_stateful_widget(list, area, &mut self.state);
        f.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}
