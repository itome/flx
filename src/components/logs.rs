use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::selector::current_session_logs::current_session_logs_selector;
use crate::redux::state::{DevTools, Focus, Home, SessionLog, State};
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct LogsComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
    wrapped_logs: HashMap<String, Vec<String>>,
    state: ListState,
}

impl LogsComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextLog.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousLog.into())?;
        Ok(())
    }

    fn wrap_text(&mut self, text: &str, n: usize) -> Vec<String> {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.wrapped_logs.entry(format!("{}_{}", n, text))
        {
            let lines = textwrap::wrap(text, n)
                .iter()
                .map(|line| line.to_string())
                .collect::<Vec<_>>();
            e.insert(lines.clone());
            lines
        } else {
            self.wrapped_logs[&format!("{}_{}", n, text)].clone()
        }
    }
}

impl Component for LogsComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::DevTools(DevTools::App) || state.popup.is_some() {
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
        let block = Block::default()
            .title("Logs")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default());

        let Some(session) = current_session_selector(state) else {
            f.render_widget(block, area);
            return;
        };
        let selected_index = session.selected_log_index.unwrap_or(0) as usize;
        self.state.select(Some(selected_index));
        let should_wrap_text = state.focus == Focus::DevTools(DevTools::App);
        let log_width = area.width as usize - 4;

        let lines = match current_session_logs_selector(state) {
            None => vec![],
            Some(logs) => logs
                .iter()
                .map(|log| match log {
                    SessionLog::Stdout(line) => {
                        if should_wrap_text {
                            let lines = self
                                .wrap_text(line, log_width)
                                .iter()
                                .map(|line| Line::raw(line.to_string()))
                                .collect::<Vec<_>>();
                            let text = Text::from(lines);
                            ListItem::new(text)
                        } else {
                            ListItem::new(line.clone())
                        }
                    }
                    SessionLog::Stderr(line) => {
                        if should_wrap_text {
                            let lines = self
                                .wrap_text(line, log_width)
                                .iter()
                                .map(|line| Line::raw(line.to_string()))
                                .collect::<Vec<_>>();
                            let text = Text::from(lines);
                            ListItem::new(text)
                        } else {
                            ListItem::new(line.clone())
                        }
                    }
                    SessionLog::Progress {
                        id,
                        message,
                        start_at,
                        end_at,
                    } => {
                        if let Some(end_at) = end_at {
                            if should_wrap_text {
                                let text = format!(
                                    "{} ({}ms)",
                                    message.clone().unwrap_or("".to_string()),
                                    end_at - start_at
                                );
                                let lines = self
                                    .wrap_text(&text, log_width)
                                    .iter()
                                    .map(|line| Line::raw(line.to_string()))
                                    .collect::<Vec<_>>();
                                let text = Text::from(lines);
                                ListItem::new(text)
                            } else {
                                ListItem::new(message.clone().unwrap_or("".to_string()))
                            }
                        } else {
                            ListItem::new(message.clone().unwrap_or("".to_string()))
                        }
                    }
                })
                .collect::<Vec<_>>(),
        };

        let mut scrollbar_state = ScrollbarState::new(lines.len()).position(selected_index);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

        let list = List::new(lines)
            .block(block)
            .highlight_style(if state.focus == Focus::DevTools(DevTools::App) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            })
            .highlight_spacing(HighlightSpacing::Never);

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
