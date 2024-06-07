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
pub struct LaunchConfigurationsComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl LaunchConfigurationsComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for LaunchConfigurationsComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let items = state
            .launch_configurations
            .iter()
            .enumerate()
            .flat_map(|(index, config)| {
                let mut items = vec![ListItem::new(format!(" {} ", config.name.clone())).bold()];
                if let Some(program) = &config.program {
                    items.push(ListItem::new(Line::from(vec![
                        Span::from("  Program   ").style(Style::default().fg(Color::Yellow)),
                        Span::from(program),
                    ])));
                }
                if let Some(mode) = &config.flutter_mode {
                    items.push(ListItem::new(Line::from(vec![
                        Span::from("  Mode      ").style(Style::default().fg(Color::Yellow)),
                        Span::from(mode),
                    ])));
                }
                if let Some(cwd) = &config.cwd {
                    items.push(ListItem::new(Line::from(vec![
                        Span::from("  Directory ").style(Style::default().fg(Color::Yellow)),
                        Span::from(cwd),
                    ])));
                }
                if let Some(args) = &config.args {
                    items.push(ListItem::new(Line::from(vec![
                        Span::from("  Args      ").style(Style::default().fg(Color::Yellow)),
                        Span::from(args.join(" ")),
                    ])));
                }
                items
            })
            .collect::<Vec<_>>();

        let block = Block::default()
            .title("Launch configurations (from .vscode/launch.json)")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        if state.launch_configurations.is_empty() {
            let text = Paragraph::new(" No configurtions found. ").block(block);
            f.render_widget(text, area);
            return;
        }

        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}
