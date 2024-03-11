use std::sync::Arc;
use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;

use crate::redux::selector::current_session_logs::CurrentSessionLogsSelector;
use crate::redux::state::{DevTools, Focus, SessionLog, State, Tab};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct LogsComponent {}

impl LogsComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for LogsComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .title("Logs")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default());

        let items = CurrentSessionLogsSelector
            .select(state)
            .iter()
            .map(|log| {
                let text = match log {
                    SessionLog::Stdout(line) => line.clone(),
                    SessionLog::Progress {
                        id,
                        message,
                        start_at,
                        end_at,
                    } => {
                        if let Some(end_at) = end_at {
                            format!(
                                "{} ({}ms)",
                                message.clone().unwrap_or("".to_string()),
                                end_at - start_at
                            )
                        } else {
                            message.clone().unwrap_or("".to_string())
                        }
                    }
                };
                ListItem::new(text)
            })
            .collect::<Vec<_>>();

        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}
