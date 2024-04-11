use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{DevTools, Focus, Home, State};
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct PerformanceComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl PerformanceComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

const UI_COLOR: Color = Color::Rgb(136, 177, 222);
const RASTER_COLOR: Color = Color::Rgb(44, 93, 170);
const UI_JANK_COLOR: Color = Color::Rgb(245, 132, 107);
const RASTER_JANK_COLOR: Color = Color::Rgb(195, 89, 90);
const MAX_FRAME_DURATION: u64 = 30;
const BAR_WIDTH: u16 = 3;
const GROUP_GAP: u16 = 1;

impl PerformanceComponent {
    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextFrame.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousFrame.into())?;
        Ok(())
    }
}

impl Component for PerformanceComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::DevTools(DevTools::Performance) || state.popup.is_some() {
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
        let border_color =
            if state.focus == Focus::DevTools(DevTools::Performance) && state.popup.is_none() {
                Color::Green
            } else {
                Color::White
            };
        let block = Block::default()
            .title("Performance")
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(border_color))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let Some(session) = current_session_selector(state) else {
            f.render_widget(block, area);
            return;
        };

        let selected_index = session
            .frames
            .iter()
            .position(|f| Some(f.number) == session.selected_frame_number);
        let mut list_state = ListState::default().with_selected(selected_index);

        let lines = session.frames.iter().map(|frame| {
            let target_ms_per_frame = 1000 / session.display_refresh_rate as u128;
            let is_ui_janky = frame.build.as_millis() > target_ms_per_frame;
            let is_raster_janky = frame.raster.as_millis() > target_ms_per_frame;
            Line::from(format!(
                "Frame: {} | Build: {}ms | Raster: {}ms",
                frame.number,
                frame.build.as_millis(),
                frame.raster.as_millis()
            ))
        });

        let mut scrollbar_state =
            ScrollbarState::new(lines.len()).position(selected_index.unwrap_or(0));
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

        let text = List::new(lines)
            .block(block)
            .highlight_style(if state.focus == Focus::DevTools(DevTools::Performance) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            })
            .highlight_spacing(HighlightSpacing::Never);

        f.render_stateful_widget(text, area, &mut list_state);
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
