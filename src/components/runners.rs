use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use crate::{
    redux::{
        action::Action,
        state::{Focus, State, Tab},
        thunk::ThunkAction,
        ActionOrThunk,
    },
    tui::Frame,
};
use color_eyre::eyre::{self, eyre, Result};
use daemon::{
    self,
    flutter::FlutterDaemon,
    io::{device::Device, event::AppMode},
};

use super::Component;

pub struct RunnersComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl Default for RunnersComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl RunnersComponent {
    pub fn new() -> Self {
        Self { action_tx: None }
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextSession.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousSession.into())?;
        Ok(())
    }

    fn show_select_device_popup(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::ShowSelectDevicePopUp.into())?;
        Ok(())
    }

    fn hot_reload(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::HotReload.into())?;
        Ok(())
    }

    fn hot_restart(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::HotRestart.into())?;
        Ok(())
    }

    fn stop_app(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::StopApp.into())?;
        Ok(())
    }
}

impl Component for RunnersComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent, state: &State) -> Result<()> {
        if state.current_focus != Focus::Tab(Tab::Runners) {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('r') => self.hot_reload()?,
            KeyCode::Char('R') => self.hot_restart()?,
            KeyCode::Char('n') => self.show_select_device_popup()?,
            KeyCode::Char('d') => self.stop_app()?,
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let default_color = if state.current_focus == Focus::Tab(Tab::Runners) {
            Color::White
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Apps")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        let items = state
            .sessions
            .iter()
            .map(|session| {
                let device = state
                    .devices
                    .iter()
                    .find(|d| d.id == session.device_id.clone().unwrap_or("".to_string()));
                let device_name = device.map(|d| d.name.clone()).unwrap_or("".to_string());
                let flavor = &session.flavor;
                let status_color = if state.current_focus != Focus::Tab(Tab::Runners) {
                    Color::DarkGray
                } else if session.hot_reloading {
                    Color::Yellow
                } else if session.hot_restarting {
                    Color::LightMagenta
                } else if !session.started {
                    Color::DarkGray
                } else {
                    Color::Green
                };
                let mut name = format!(" {} ", device_name);
                if let Some(flavor) = flavor {
                    name.push_str(&format!("({})", flavor))
                }
                let item = ListItem::new(name).style(Style::default().fg(status_color));
                if state.session_id == Some(session.id.clone()) {
                    item.add_modifier(Modifier::REVERSED)
                        .add_modifier(Modifier::BOLD)
                } else {
                    item
                }
            })
            .collect::<Vec<_>>();

        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}
