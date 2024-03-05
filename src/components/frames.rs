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

const UI_COLOR: Color = Color::Rgb(136, 177, 222);
const RASTER_COLOR: Color = Color::Rgb(44, 93, 170);
const UI_JANK_COLOR: Color = Color::Rgb(245, 132, 107);
const RASTER_JANK_COLOR: Color = Color::Rgb(195, 89, 90);
const MAX_FRAME_DURATION: u64 = 30;
const BAR_WIDTH: u16 = 4;
const GROUP_GAP: u16 = 1;

impl Component for FramesComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .title("Frames")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL);

        let Some(session) = CurrentSessionSelector.select(state) else {
            f.render_widget(block, area);
            return;
        };

        let max_frame_count = ((area.width - 2) / (BAR_WIDTH * 2 + GROUP_GAP) as u16) as usize;
        let skip = if session.frames.len() > max_frame_count {
            session.frames.len() - max_frame_count
        } else {
            0
        };

        let frame_groups = session
            .frames
            .iter()
            .skip(skip)
            .enumerate()
            .map(|(index, frame)| {
                let ui_bar = Bar::default()
                    .value(frame.build.as_millis() as u64)
                    .text_value("".to_string())
                    .style(Style::default().fg(UI_COLOR));
                let raster_bar = Bar::default()
                    .value(frame.raster.as_millis() as u64)
                    .text_value("".to_string())
                    .style(Style::default().fg(RASTER_COLOR));
                let bar_group = BarGroup::default().bars(&[ui_bar, raster_bar]);
                if index % 2 == 1 {
                    bar_group.label(Line::from(frame.number.to_string()).centered())
                } else {
                    bar_group
                }
            });

        let mut barchart = BarChart::default()
            .block(block)
            .bar_width(BAR_WIDTH)
            .bar_gap(0)
            .group_gap(GROUP_GAP)
            .max(MAX_FRAME_DURATION);

        for group in frame_groups {
            barchart = barchart.data(group);
        }

        f.render_widget(barchart, area);
    }
}
