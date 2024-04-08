use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{Focus, Home, State};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct FrameAnalysisComponent {}

impl FrameAnalysisComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for FrameAnalysisComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .title("Frame Analysis")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let todo = Paragraph::new("Todo").block(block);
        f.render_widget(todo, area);
    }
}
