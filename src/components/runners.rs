use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use redux_rs::{middlewares::thunk::ActionOrThunk, StoreApi};
use tokio::sync::Mutex;

use crate::{
    daemon::{self, flutter::FlutterDaemon, io::device::Device},
    redux::state::{State, Tab},
    tui::Frame,
};
use color_eyre::eyre::Result;

use super::Component;

pub struct RunnersComponent {
    list_state: ListState,
}

impl RunnersComponent {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    fn next(&mut self) {
        // let i = match self.list_state.selected() {
        //     Some(i) => {
        //         let sessions = &self.session_manager.lock().unwrap().sessions;
        //         if i >= sessions.len() {
        //             0
        //         } else {
        //             i + 1
        //         }
        //     }
        //     None => 0,
        // };
        // self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        // let i = match self.list_state.selected() {
        //     Some(i) => {
        //         if i == 0 {
        //             let sessions = &self.session_manager.lock().unwrap().sessions;
        //             sessions.len()
        //         } else {
        //             i - 1
        //         }
        //     }
        //     None => 0,
        // };
        // self.list_state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.list_state.select(None);
    }

    fn run_new_app(&mut self) -> Result<()> {
        // if let Ok(mut session_manager) = self.session_manager.lock() {
        //     session_manager.run_new_app()?;
        // }
        Ok(())
    }
}

impl Component for RunnersComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let default_color = if state.selected_tab == Tab::Runners {
            Color::White
        } else {
            Color::DarkGray
        };
        let enabled_color = if state.selected_tab == Tab::Runners {
            Color::Green
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Apps")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(default_color));

        let sessions: Vec<String> = vec![];
        let mut items = sessions
            .iter()
            .enumerate()
            .map(|(index, _)| {
                ListItem::new(format!(" App {} ", index + 1))
                    .style(Style::default().fg(enabled_color))
            })
            .collect::<Vec<_>>();
        items.push(ListItem::new(" ▶ Run new app ").style(Style::default().fg(default_color)));

        let list = List::new(items)
            .block(block)
            .fg(Color::White)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_stateful_widget(list, area, &mut self.list_state);
    }
}
