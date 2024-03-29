use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::redux::action::Action;
use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{Focus, State, Tab};
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct InspectorComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl InspectorComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl InspectorComponent {
    fn next(&self) -> Result<()> {
        todo!()
    }

    fn previous(&self) -> Result<()> {
        todo!()
    }
}

impl Component for InspectorComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent, state: &State) -> Result<()> {
        if state.current_focus != Focus::Tab(Tab::Inspector) {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color = if state.current_focus == Focus::Tab(Tab::Performance) {
            Color::Green
        } else {
            Color::White
        };
        let block = Block::default()
            .title("Flutter Inspector")
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(border_color))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        f.render_widget(block, area);
    }
}
