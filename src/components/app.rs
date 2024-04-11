use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{DevTools, Focus, State};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct AppComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl AppComponent {
    pub fn new() -> Self {
        Self::default()
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
}

impl Component for AppComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if !matches!(state.focus, Focus::DevTools(_)) || state.popup.is_some() {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('r') => self.hot_reload()?,
            KeyCode::Char('R') => self.hot_restart()?,
            _ => {}
        }
        Ok(())
    }

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
