use crate::redux::action::Action;
use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{DevTools, Focus, Home, State};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyEvent};
use daemon::flutter::FlutterDaemon;
use daemon::io::request;
use devtools::protocols::io_extension::{HttpProfileRequest, HttpProfileRequestRef};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use serde_json::{Map, Value};
use std::default;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use url::Url;

use super::Component;

#[derive(PartialEq)]
enum Tab {
    Headers,
    Payload,
    Response,
    Timing,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Headers
    }
}

#[derive(Default)]
pub struct NetworkRequestComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
    headers_table_state: TableState,
    payload_list_state: ListState,
    response_list_state: ListState,
    selected_tab: Tab,
}

impl NetworkRequestComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            Tab::Headers => Tab::Payload,
            Tab::Payload => Tab::Response,
            Tab::Response => Tab::Timing,
            Tab::Timing => Tab::Headers,
        };
    }

    fn previous_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            Tab::Headers => Tab::Timing,
            Tab::Payload => Tab::Headers,
            Tab::Response => Tab::Payload,
            Tab::Timing => Tab::Response,
        };
    }

    fn next_headers(&mut self, state: &State) {
        let Some(session) = CurrentSessionSelector.select(state) else {
            return;
        };
        let Some(request) = session
            .requests
            .iter()
            .find(|r| Some(r.id.clone()) == session.selected_request_id)
        else {
            return;
        };
        let general = Self::format_general(request);
        let request_headers = Self::format_headers(
            request
                .request
                .clone()
                .map(|r| r.headers.unwrap_or_default())
                .unwrap_or_default(),
        );
        let response_headers = Self::format_headers(
            request
                .response
                .clone()
                .map(|r| r.headers.unwrap_or_default())
                .unwrap_or_default(),
        );
        let length = general.len() + request_headers.len() + response_headers.len() + 3; // for headers

        self.headers_table_state.select(Some(
            self.headers_table_state
                .selected()
                .map(|i| if i + 1 < length { i + 1 } else { i })
                .unwrap_or_default(),
        ));
    }

    fn previous_headers(&mut self) {
        self.headers_table_state.select(Some(
            self.headers_table_state
                .selected()
                .map(|i| i.saturating_sub(1))
                .unwrap_or_default(),
        ));
    }

    fn next_payload(&mut self, state: &State) {
        let Some(session) = CurrentSessionSelector.select(state) else {
            return;
        };
        let Some(selected_request_id) = session.selected_request_id.clone() else {
            return;
        };
        let Some(request) = session.full_requests.get(&selected_request_id) else {
            return;
        };
        let Some(body) = request.request_body.clone() else {
            return;
        };
        let Ok(body) = String::from_utf8(body) else {
            return;
        };
        let length = body.split("\n").collect::<Vec<_>>().len();
        self.payload_list_state.select(Some(
            self.payload_list_state
                .selected()
                .map(|i| if i + 1 < length { i + 1 } else { i })
                .unwrap_or_default(),
        ));
    }

    fn previous_payload(&mut self) {
        self.payload_list_state.select(Some(
            self.payload_list_state
                .selected()
                .map(|i| i.saturating_sub(1))
                .unwrap_or_default(),
        ));
    }

    fn next_response(&mut self, state: &State) {
        let Some(session) = CurrentSessionSelector.select(state) else {
            return;
        };
        let Some(selected_request_id) = session.selected_request_id.clone() else {
            return;
        };
        let Some(request) = session.full_requests.get(&selected_request_id) else {
            return;
        };
        let Some(body) = request.response_body.clone() else {
            return;
        };
        let Ok(body) = String::from_utf8(body) else {
            return;
        };
        let length = body.split("\n").collect::<Vec<_>>().len();
        self.response_list_state.select(Some(
            self.response_list_state
                .selected()
                .map(|i| if i + 1 < length { i + 1 } else { i })
                .unwrap_or_default(),
        ));
    }

    fn previous_response(&mut self) {
        self.response_list_state.select(Some(
            self.response_list_state
                .selected()
                .map(|i| i.saturating_sub(1))
                .unwrap_or_default(),
        ));
    }

    fn load_full_request(&mut self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::LoadFullRequest.into())?;
        Ok(())
    }

    fn exit_network_request(&mut self) -> Result<()> {
        self.selected_tab = Tab::Headers;
        self.headers_table_state.select(None);
        self.payload_list_state.select(None);
        self.response_list_state.select(None);
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::ExitNetworkRequest.into())?;
        Ok(())
    }

    fn format_general(request: &HttpProfileRequestRef) -> Vec<(String, String)> {
        vec![
            ("Request URL".to_string(), request.uri.clone()),
            ("Request Method".to_string(), request.method.clone()),
            (
                "Status Code".to_string(),
                request
                    .response
                    .clone()
                    .map(|r| {
                        r.status_code
                            .map(|s| s.to_string().to_uppercase())
                            .unwrap_or("-".to_string())
                    })
                    .unwrap_or("-".to_string()),
            ),
            (
                "Connection Info".to_string(),
                request
                    .response
                    .clone()
                    .map(|r| {
                        r.connection_info
                            .map(|c| serde_json::to_string(&c).unwrap_or("-".to_string()))
                            .unwrap_or("-".to_string())
                    })
                    .unwrap_or("-".to_string()),
            ),
            (
                "Content Type".to_string(),
                request
                    .response
                    .clone()
                    .map(|r| {
                        r.headers
                            .map(|h| {
                                h.get("content-type")
                                    .map(|c| c.to_string())
                                    .unwrap_or("-".to_string())
                            })
                            .unwrap_or("-".to_string())
                    })
                    .unwrap_or("-".to_string()),
            ),
        ]
    }

    fn format_headers(headers: Map<String, Value>) -> Vec<(String, String)> {
        headers
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    match v {
                        Value::Array(list) if list.len() != 1 => v.to_string(),
                        Value::Array(list) => list
                            .first()
                            .map(|e| e.to_string())
                            .unwrap_or("".to_string()),
                        _ => v.to_string(),
                    },
                )
            })
            .collect::<Vec<(String, String)>>()
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

    fn draw_headers(
        &mut self,
        f: &mut Frame<'_>,
        area: Rect,
        scrollbar_area: Rect,
        request: &HttpProfileRequestRef,
    ) {
        let general = Self::format_general(request);
        let request_headers = Self::format_headers(
            request
                .request
                .clone()
                .map(|r| r.headers.unwrap_or_default())
                .unwrap_or_default(),
        );
        let response_headers = Self::format_headers(
            request
                .response
                .clone()
                .map(|r| r.headers.unwrap_or_default())
                .unwrap_or_default(),
        );

        let mut rows: Vec<Row> = vec![Row::new(vec![
            Cell::from(" General").style(Style::default().fg(Color::Yellow).bold())
        ])];
        for (k, v) in general {
            rows.push(Row::new(vec![Cell::new(format!("   {}", k)), Cell::new(v)]));
        }
        if !response_headers.is_empty() {
            rows.push(Row::new(vec![
                Cell::from(" Response Headers").style(Style::default().fg(Color::Yellow).bold())
            ]));
            for (k, v) in response_headers {
                rows.push(Row::new(vec![Cell::new(format!("   {}", k)), Cell::new(v)]));
            }
        }
        if !request_headers.is_empty() {
            rows.push(Row::new(vec![
                Cell::from(" Request Headers").style(Style::default().fg(Color::Yellow).bold())
            ]));
            for (k, v) in request_headers {
                rows.push(Row::new(vec![Cell::new(format!("   {}", k)), Cell::new(v)]));
            }
        }

        let mut scrollbar_state = ScrollbarState::new(rows.len())
            .position(self.headers_table_state.selected().unwrap_or(0));
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let widths = [Constraint::Fill(1), Constraint::Fill(3)];
        let list = Table::new(rows, widths)
            .highlight_spacing(HighlightSpacing::Never)
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(list, area, &mut self.headers_table_state);
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    fn draw_payload(
        &mut self,
        f: &mut Frame<'_>,
        area: Rect,
        scrollbar_area: Rect,
        request: &Option<&HttpProfileRequest>,
    ) {
        let Some(request) = request else {
            return;
        };
        let Some(body) = request.request_body.clone() else {
            f.render_widget(Text::raw(" No request body "), area);
            return;
        };
        if body.is_empty() {
            f.render_widget(Text::raw(" No request body "), area);
            return;
        }
        let Ok(body) = String::from_utf8(body) else {
            return;
        };
        let items = body
            .split("\n")
            .map(|l| ListItem::new(l))
            .collect::<Vec<_>>();
        let mut scrollbar_state = ScrollbarState::new(items.len())
            .position(self.payload_list_state.selected().unwrap_or(0));
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let list = List::new(items)
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_spacing(HighlightSpacing::Never);

        f.render_stateful_widget(
            list,
            area.inner(&Margin {
                vertical: 0,
                horizontal: 1,
            }),
            &mut self.payload_list_state,
        );
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    fn draw_response(
        &mut self,
        f: &mut Frame<'_>,
        area: Rect,
        scrollbar_area: Rect,
        request: &Option<&HttpProfileRequest>,
    ) {
        let Some(request) = request else {
            return;
        };
        let Some(body) = request.response_body.clone() else {
            f.render_widget(Text::raw(" No response body "), area);
            return;
        };
        if body.is_empty() {
            f.render_widget(Text::raw(" No response body "), area);
            return;
        }
        let Ok(body) = String::from_utf8(body) else {
            return;
        };
        let items = body
            .split("\n")
            .map(|l| ListItem::new(l))
            .collect::<Vec<_>>();
        let mut scrollbar_state = ScrollbarState::new(items.len())
            .position(self.response_list_state.selected().unwrap_or(0));
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let list = List::new(items)
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_spacing(HighlightSpacing::Never);
        f.render_stateful_widget(
            list,
            area.inner(&Margin {
                vertical: 0,
                horizontal: 1,
            }),
            &mut self.response_list_state,
        );
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    fn draw_timing(
        &mut self,
        f: &mut Frame<'_>,
        area: Rect,
        scrollbar_area: Rect,
        request: &HttpProfileRequestRef,
    ) {
        let events = request
            .request
            .clone()
            .map(|r| r.events)
            .unwrap_or_default();

        let mut rows: Vec<Row> = vec![];
        let mut start = request.start_time;
        for event in events {
            rows.push(Row::new(vec![
                Cell::new(format!("  {}", event.event)),
                Cell::new(format!(
                    "{: >7}",
                    Self::format_duration(Duration::from_micros((event.timestamp - start) as u64,))
                )),
            ]));
            start = event.timestamp;
        }
        if let Some(end_time) = request.end_time {
            rows.push(Row::new(vec![
                Cell::new("  Total"),
                Cell::new(format!(
                    "{: >7}",
                    Self::format_duration(Duration::from_micros(
                        (end_time - request.start_time) as u64,
                    ))
                )),
            ]));
        }
        let widths = [Constraint::Fill(1), Constraint::Fill(3)];
        let list = Table::new(rows, widths);

        f.render_widget(list, area);
    }
}

impl Component for NetworkRequestComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::DevTools(DevTools::NetworkRequest) || state.popup.is_some() {
            return Ok(());
        }
        match key.code {
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => self.next_tab(),
            KeyCode::Left | KeyCode::Char('h') | KeyCode::BackTab => self.previous_tab(),
            KeyCode::Esc => self.exit_network_request()?,
            _ => {}
        }
        match self.selected_tab {
            Tab::Headers => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.previous_headers(),
                KeyCode::Down | KeyCode::Char('j') => self.next_headers(state),
                _ => {}
            },
            Tab::Payload => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.previous_payload(),
                KeyCode::Down | KeyCode::Char('j') => self.next_payload(state),
                _ => {}
            },
            Tab::Response => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.previous_response(),
                KeyCode::Down | KeyCode::Char('j') => self.next_response(state),
                _ => {}
            },
            _ => {}
        }
        if self.selected_tab == Tab::Payload || self.selected_tab == Tab::Response {
            self.load_full_request()?;
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color =
            if state.focus == Focus::DevTools(DevTools::NetworkRequest) && state.popup.is_none() {
                Color::Green
            } else {
                Color::White
            };
        let empty = Paragraph::new("No request selected")
            .style(Style::default().fg(border_color))
            .block(
                Block::default()
                    .padding(Padding::horizontal(1))
                    .border_style(Style::default().fg(border_color))
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            );
        let Some(session) = CurrentSessionSelector.select(state) else {
            f.render_widget(empty, area);
            return;
        };
        let Some(network_request) = session
            .requests
            .iter()
            .find(|r| Some(r.id.clone()) == session.selected_request_id)
        else {
            f.render_widget(empty, area);
            return;
        };

        let full_request = session.full_requests.get(&network_request.id);

        let block = Block::default()
            .title_bottom(Line::from(r#"Press "Tab" to select tabs"#).right_aligned())
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .borders(Borders::ALL);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(block.inner(area));

        let tabs = Tabs::new(vec!["Headers", "Payload", "Response", "Timing"])
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(Style::default().fg(border_color).bold())
            .select(match self.selected_tab {
                Tab::Headers => 0,
                Tab::Payload => 1,
                Tab::Response => 2,
                Tab::Timing => 3,
            });

        f.render_widget(block, area);
        f.render_widget(tabs, layout[0]);
        f.render_widget(
            Block::new()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(border_color)),
            layout[1],
        );

        match self.selected_tab {
            Tab::Headers => self.draw_headers(
                f,
                layout[2],
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &network_request,
            ),
            Tab::Payload => self.draw_payload(
                f,
                layout[2],
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &full_request,
            ),
            Tab::Response => self.draw_response(
                f,
                layout[2],
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &full_request,
            ),
            Tab::Timing => self.draw_timing(
                f,
                layout[2],
                area.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &network_request,
            ),
        }
    }
}
