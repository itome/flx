use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use devtools::protocols::flutter_extension::DiagnosticNode;
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{DevTools, Focus, Home, State};
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct InspectorComponent<'a> {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
    state: TreeState<String>,
    items: Vec<TreeItem<'a, String>>,
}

impl<'a> InspectorComponent<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn item_builder(item: &DiagnosticNode) -> TreeItem<'a, String> {
        if let Some(children) = item.children.as_ref() {
            let children = children.iter().map(Self::item_builder).collect();
            TreeItem::new(
                item.value_id.clone().unwrap_or_default(),
                item.description.clone().unwrap_or_default(),
                children,
            )
            .unwrap()
        } else {
            TreeItem::new_leaf(
                item.value_id.clone().unwrap_or_default(),
                item.description.clone().unwrap_or_default(),
            )
        }
    }

    fn next(&mut self) {
        self.state.key_down(&self.items);
    }

    fn previous(&mut self) {
        self.state.key_up(&self.items);
    }

    fn open(&mut self) {
        self.state.key_right();
    }

    fn close(&mut self) {
        self.state.key_left();
    }

    fn toggle(&mut self) {
        self.state.toggle_selected();
    }
}

impl<'a> Component for InspectorComponent<'a> {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.next(),
            KeyCode::Char('k') | KeyCode::Up => self.previous(),
            KeyCode::Char('l') | KeyCode::Right => self.open(),
            KeyCode::Char('h') | KeyCode::Left => self.close(),
            KeyCode::Enter => self.toggle(),
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color =
            if state.focus == Focus::DevTools(DevTools::Inspector) && state.popup.is_none() {
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

        let Some(session) = current_session_selector(state) else {
            f.render_widget(block.clone(), area);
            return;
        };

        self.items = session
            .widget_summary_tree
            .iter()
            .map(Self::item_builder)
            .collect::<Vec<_>>();

        let tree = Tree::new(self.items.clone())
            .unwrap()
            .block(block)
            .experimental_scrollbar(Some(Scrollbar::new(ScrollbarOrientation::VerticalRight)))
            .highlight_style(if state.focus == Focus::DevTools(DevTools::Inspector) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            });

        f.render_stateful_widget(tree, area, &mut self.state);
    }
}
