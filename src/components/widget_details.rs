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
use crate::redux::state::{DevTools, Focus, Home, SessionState, State};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use crate::widgets::tree::{Node, Tree, TreeState};
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct WidgetDetailsComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
    state: TreeState,
}

impl WidgetDetailsComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn item_builder(item: &DiagnosticNode) -> Node {
        let value_id = item.value_id.clone().unwrap_or_default();

        let spans = if let Some(name) = item.name.clone() {
            vec![
                Span::styled(format!("{}: ", name), Style::default().fg(Color::Yellow)),
                Span::raw(item.description.clone().unwrap_or_default()),
            ]
        } else {
            vec![Span::styled(
                item.description.clone().unwrap_or_default(),
                Style::default().bold(),
            )]
        };

        let mut child_nodes: Vec<Node> = vec![];

        if let Some(properties) = item.properties.as_ref() {
            child_nodes.extend(properties.iter().map(Self::item_builder));
        }

        if let Some(children) = item.children.as_ref() {
            child_nodes.extend(children.iter().map(Self::item_builder));
        }

        Node::new(&value_id, spans, child_nodes)
    }

    fn next(&mut self, state: &State) {
        let Some(session) = current_session_selector(state) else {
            return;
        };

        let Some(ref details_tree) = session.selected_widget_details_tree else {
            return;
        };
        let root = Self::item_builder(details_tree);
        let state = TreeState::new()
            .with_opened(session.opened_widget_details_value_ids.clone())
            .with_selected(self.state.selected());
        let paths = root.flatten(&state.opened, &[]);

        let current_index = paths.iter().position(|path| {
            if let Some(selected) = self.state.selected.as_ref() {
                path.last().unwrap() == selected
            } else {
                false
            }
        });

        let next_id = if let Some(current_index) = current_index {
            if current_index + 1 < paths.len() {
                Some(paths[current_index + 1].last().unwrap().clone())
            } else {
                self.state.selected.clone()
            }
        } else {
            paths.first().map(|path| path.last().unwrap().clone())
        };

        if let Some(next_id) = next_id {
            self.state.selected.clone_from(&Some(next_id));
        }
    }

    fn previous(&mut self, state: &State) {
        let Some(session) = current_session_selector(state) else {
            return;
        };

        let Some(ref details_tree) = session.selected_widget_details_tree else {
            return;
        };
        let root = Self::item_builder(details_tree);
        let state = TreeState::new()
            .with_opened(session.opened_widget_details_value_ids.clone())
            .with_selected(self.state.selected());
        let paths = root.flatten(&state.opened, &[]);

        let current_index = paths.iter().position(|path| {
            if let Some(selected) = self.state.selected.as_ref() {
                path.last().unwrap() == selected
            } else {
                false
            }
        });

        let next_id = if let Some(current_index) = current_index {
            if current_index > 0 {
                Some(paths[current_index - 1].last().unwrap().clone())
            } else {
                self.state.selected.clone()
            }
        } else {
            paths.first().map(|path| path.last().unwrap().clone())
        };

        if let Some(next_id) = next_id {
            self.state.selected.clone_from(&Some(next_id));
        }
    }

    fn toggle(&mut self, state: &State) {
        let Some(session) = current_session_selector(state) else {
            return;
        };
        let Some(selected_widget_id) = self.state.selected.as_ref() else {
            return;
        };
        self.action_tx
            .as_ref()
            .unwrap()
            .send(
                Action::ToggleOpenWidgetDetailsValueId {
                    session_id: session.id.clone(),
                    id: selected_widget_id.clone(),
                }
                .into(),
            )
            .unwrap();
    }

    fn exit_widget_details(&mut self) -> Result<()> {
        self.state.selected.clone_from(&None);
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::ExitWidgetDetails.into())?;
        Ok(())
    }
}

impl Component for WidgetDetailsComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::DevTools(DevTools::WidgetDetails) || state.popup.is_some() {
            return Ok(());
        }

        match key.code {
            KeyCode::Esc => self.exit_widget_details()?,
            KeyCode::Char('j') | KeyCode::Down => self.next(state),
            KeyCode::Char('k') | KeyCode::Up => self.previous(state),
            KeyCode::Tab => self.toggle(state),
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color =
            if state.focus == Focus::DevTools(DevTools::WidgetDetails) && state.popup.is_none() {
                Color::Green
            } else {
                Color::White
            };
        let block = Block::default()
            .title("Widget Details Tree")
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(border_color))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let Some(SessionState {
            selected_widget_details_tree: Some(ref details_tree),
            opened_widget_details_value_ids,
            ..
        }) = current_session_selector(state)
        else {
            f.render_widget(block.clone(), area);
            return;
        };

        let root = Self::item_builder(details_tree);
        let tree = Tree::new(root).block(block).highlight_style(
            if state.focus == Focus::DevTools(DevTools::WidgetDetails) {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            },
        );

        self.state
            .opened
            .clone_from(&opened_widget_details_value_ids);

        f.render_stateful_widget(tree, area, &mut self.state);
    }
}
