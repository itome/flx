use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::{mpsc::UnboundedSender, Mutex};

use crate::{
    daemon::{self, flutter::FlutterDaemon, io::device::Device},
    redux::{
        action::Action,
        state::{State, Tab},
        thunk::ThunkAction,
        ActionOrThunk,
    },
    tui::Frame,
};
use color_eyre::eyre::{self, eyre, Result};

use super::Component;

pub struct RunnersComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
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

    fn run_new_app(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::RunNewApp.into())?;
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
}

impl Component for RunnersComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<crate::action::TuiAction>> {
        match key.code {
            KeyCode::Char('r') => self.hot_reload()?,
            KeyCode::Char('R') => self.hot_restart()?,
            KeyCode::Char('n') => self.run_new_app()?,
            KeyCode::Up => self.previous()?,
            KeyCode::Down => self.next()?,
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let default_color = if state.selected_tab == Tab::Runners {
            Color::White
        } else {
            Color::DarkGray
        };
        let enabled_color = if state.selected_tab == Tab::Runners {
            Color::Green
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Apps")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        let mut items = state
            .sessions
            .iter()
            .map(|session_id| {
                ListItem::new(session_id.clone()).style(Style::default().fg(enabled_color))
            })
            .collect::<Vec<_>>();
        items.push(ListItem::new(" ▶ Run new app ").style(Style::default().fg(default_color)));

        let list = List::new(items)
            .block(block)
            .fg(Color::White)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_widget(list, area);
    }
}
