use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{DevTools, Focus, Home, State};
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use crossterm::event::{KeyCode, KeyEvent};
use daemon::flutter::FlutterDaemon;
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use url::Url;

use super::Component;

#[derive(Default)]
pub struct NetworkComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl NetworkComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextReqest.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousRequest.into())?;
        Ok(())
    }

    fn enter_network_request(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::EnterNetworkRequest.into())?;
        Ok(())
    }

    fn format_duration(duration: Duration) -> String {
        if duration.as_millis() < 1000 {
            format!("{}ms", duration.as_millis())
        } else {
            format!(
                "{:.1}s",
                duration.as_secs() as f64 + duration.as_millis() as f64 / 1000.0
            )
        }
    }
}

impl Component for NetworkComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::DevTools(DevTools::Network) || state.popup.is_some() {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            KeyCode::Enter => self.enter_network_request()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color =
            if state.focus == Focus::DevTools(DevTools::Network) && state.popup.is_none() {
                Color::Green
            } else {
                Color::White
            };
        let block = Block::default()
            .title("Network")
            .padding(Padding::horizontal(1))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .borders(Borders::ALL);

        let Some(session) = current_session_selector(state) else {
            f.render_widget(block, area);
            return;
        };

        let selected_index = if let Some(selected_request_id) = &session.selected_request_id {
            session
                .requests
                .iter()
                .position(|r| &r.id == selected_request_id)
        } else {
            None
        };
        let mut table_state = TableState::default().with_selected(selected_index);

        let widths = [
            Constraint::Length(7),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(8),
        ];
        let rows = session.requests.iter().map(|request| {
            let statu_code = match &request.response {
                Some(response) => response.status_code,
                None => None,
            };
            let status_color = match statu_code {
                Some(code) if (200..300).contains(&code) => Color::Blue,
                Some(code) if (300..400).contains(&code) => Color::Yellow,
                Some(code) if (400..600).contains(&code) => Color::Red,
                _ => Color::White,
            };
            let last_uri_path_and_query_string = match Url::parse(&request.uri) {
                Ok(url) => match url.path_segments() {
                    Some(path) => [
                        path.last().unwrap_or("-").to_string(),
                        url.query().map(|q| format!("?{}", q)).unwrap_or_default(),
                    ]
                    .join(""),
                    None => "-".to_string(),
                },
                Err(_) => "-".to_string(),
            };
            let method_color = match request.method.as_str() {
                "GET" => Color::Green,
                "POST" => Color::Yellow,
                "PUT" => Color::Blue,
                "DELETE" => Color::Red,
                _ => Color::White,
            };
            let time = match &request.request {
                Some(req) => match req.events.last() {
                    Some(last_event) => Duration::from_micros(
                        (last_event.timestamp - request.start_time).unsigned_abs(),
                    ),
                    _ => std::time::Duration::default(),
                },
                _ => std::time::Duration::default(),
            };
            let cells = vec![
                Cell::from(Span::styled(
                    request.method.clone(),
                    Style::default().fg(method_color).bold(),
                )),
                Cell::from(Span::styled(
                    last_uri_path_and_query_string,
                    Style::default(),
                )),
                Cell::from(Span::styled(
                    statu_code
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                    Style::default().fg(status_color),
                )),
                Cell::from(Span::raw(format!("{: >7}", Self::format_duration(time)))),
            ];
            Row::new(cells)
        });

        let mut scrollbar_state =
            ScrollbarState::new(rows.len()).position(selected_index.unwrap_or(0));
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

        let table = Table::new(rows, widths)
            .block(block)
            .highlight_style(
                if state.focus == Focus::DevTools(DevTools::Network)
                    || state.focus == Focus::DevTools(DevTools::NetworkRequest)
                {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                },
            )
            .highlight_spacing(HighlightSpacing::Never);

        f.render_stateful_widget(table, area, &mut table_state);
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
