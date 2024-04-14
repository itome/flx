use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::TuiAction;
use crate::redux::action::Action;
use crate::redux::selector::selected_device::selected_device_selector;
use crate::redux::state::{Home, PopUp, State};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct SdkVersionComponent {}

impl SdkVersionComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for SdkVersionComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .title("SDK Version")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_type(BorderType::Rounded);

        let Some(version) = state.sdk_version.as_ref() else {
            f.render_widget(block.clone(), area);
            return;
        };

        let widths = [Constraint::Length(21), Constraint::Fill(1)];

        let rows = vec![
            Row::new(vec![
                Cell::from("Flutter Version").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.flutter_version.clone()),
            ]),
            Row::new(vec![
                Cell::from("Framework Version").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.framework_version.clone()),
            ]),
            Row::new(vec![
                Cell::from("Channel").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.channel.clone()),
            ]),
            Row::new(vec![
                Cell::from("Repository URL").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.repository_url.clone()),
            ]),
            Row::new(vec![
                Cell::from("Framework Revision").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.framework_revision.clone()),
            ]),
            Row::new(vec![
                Cell::from("Framework Commit Date").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.framework_commit_date.clone()),
            ]),
            Row::new(vec![
                Cell::from("Engine Revision").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.engine_revision.clone()),
            ]),
            Row::new(vec![
                Cell::from("Dart SDK Version").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.dart_sdk_version.clone()),
            ]),
            Row::new(vec![
                Cell::from("Flutter Root").style(Style::default().fg(Color::Yellow)),
                Cell::from(version.flutter_root.clone()),
            ]),
        ];

        let table = Table::new(rows, widths).block(block);

        f.render_widget(table, area);
    }
}
