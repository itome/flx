use std::collections::HashSet;
use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use devtools::protocols::flutter_extension::DiagnosticNode;
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{DevTools, Focus, Home, State};
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use crate::widgets::tree::{Node, Tree, TreeState};
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

pub struct InspectorComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
    state: TreeState,

    finished_first_render: bool,
}

impl<'a> InspectorComponent {
    pub fn new() -> Self {
        Self {
            action_tx: None,
            state: TreeState::new(),
            finished_first_render: false,
        }
    }

    fn item_builder(item: &DiagnosticNode) -> Node<'a> {
        if let Some(children) = item.children.as_ref() {
            let children = children.iter().map(Self::item_builder).collect();
            Node::new(
                &item.value_id.clone().unwrap_or_default(),
                vec![Span::raw(item.description.clone().unwrap_or_default())],
                children,
            )
        } else {
            Node::new(
                &item.value_id.clone().unwrap_or_default(),
                vec![Span::raw(item.description.clone().unwrap_or_default())],
                vec![],
            )
        }
    }

    fn next(&mut self, state: &State) {
        let Some(session) = current_session_selector(state) else {
            return;
        };

        let Some(ref summary_tree) = session.widget_summary_tree else {
            return;
        };
        let root = Self::item_builder(summary_tree);
        let items = Tree::make_lines(&root, &self.state, &vec![], &vec![]);

        let current_index = items.iter().position(|(id, _)| {
            if let Some(selected) = session.selected_widget_value_id.as_ref() {
                id == selected
            } else {
                false
            }
        });

        let next_id = if let Some(current_index) = current_index {
            if current_index + 1 < items.len() {
                Some(items[current_index + 1].0.clone())
            } else {
                session.selected_widget_value_id.clone()
            }
        } else {
            items.first().map(|(id, _)| id.clone())
        };

        if let Some(next_id) = next_id {
            self.action_tx
                .as_ref()
                .unwrap()
                .send(
                    Action::SelectWidgetValueId {
                        session_id: session.id.clone(),
                        id: next_id,
                    }
                    .into(),
                )
                .unwrap();
        }
    }

    fn previous(&mut self, state: &State) {
        let Some(session) = current_session_selector(state) else {
            return;
        };

        let Some(ref summary_tree) = session.widget_summary_tree else {
            return;
        };
        let root = Self::item_builder(summary_tree);
        let items = Tree::make_lines(&root, &self.state, &vec![], &vec![]);

        let current_index = items.iter().position(|(id, _)| {
            if let Some(selected) = session.selected_widget_value_id.as_ref() {
                id == selected
            } else {
                false
            }
        });

        let next_id = if let Some(current_index) = current_index {
            if current_index > 0 {
                Some(items[current_index - 1].0.clone())
            } else {
                session.selected_widget_value_id.clone()
            }
        } else {
            items.first().map(|(id, _)| id.clone())
        };

        if let Some(next_id) = next_id {
            self.action_tx
                .as_ref()
                .unwrap()
                .send(
                    Action::SelectWidgetValueId {
                        session_id: session.id.clone(),
                        id: next_id,
                    }
                    .into(),
                )
                .unwrap();
        }
    }

    fn toggle(&mut self, state: &State) {
        let Some(session) = current_session_selector(state) else {
            return;
        };
        let Some(selected_widget_id) = session.selected_widget_value_id.as_ref() else {
            return;
        };
        self.state.toggle(selected_widget_id);
    }

    fn all_ids(node: &DiagnosticNode) -> HashSet<String> {
        let mut ids = HashSet::new();
        ids.insert(node.value_id.clone().unwrap_or_default());
        if let Some(children) = node.children.as_ref() {
            for child in children {
                ids.extend(Self::all_ids(child));
            }
        }
        ids
    }
}

impl Component for InspectorComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.next(state),
            KeyCode::Char('k') | KeyCode::Up => self.previous(state),
            KeyCode::Enter => self.toggle(state),
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

        let Some(ref summary_tree) = session.widget_summary_tree else {
            f.render_widget(block.clone(), area);
            return;
        };

        if !self.finished_first_render {
            self.finished_first_render = true;
            let ids = Self::all_ids(summary_tree);
            self.state.open_all(ids);
        }

        let root = Self::item_builder(summary_tree);
        let tree = Tree::new(root).block(block).highlight_style(
            if state.focus == Focus::DevTools(DevTools::Inspector) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            },
        );

        if let Some(selected_widget_value_id) = session.selected_widget_value_id.as_ref() {
            *self.state.selected_mut() = Some(selected_widget_value_id.clone());
        }

        f.render_stateful_widget(tree, area, &mut self.state);
    }
}
