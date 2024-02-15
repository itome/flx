use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;

use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{Focus, State, Tab};
use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::Result;

use super::Component;

#[derive(Default)]
pub struct FramesComponent {}

impl FramesComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for FramesComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .title("Frames")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL);

        f.render_widget(block, area);
    }
}
