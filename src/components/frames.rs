use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;

use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{Focus, State, Tab};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

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
const BAR_WIDTH: u16 = 3;
const GROUP_GAP: u16 = 1;

impl Component for FramesComponent {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let block = Block::default()
            .title("Frames")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL);

        let ledgend_width = "Raster Jank".len() as u16 + 2;
        let ledgend_area = Rect {
            height: 6,
            width: ledgend_width,
            y: area.y,
            x: area.right() - ledgend_width,
        };

        let Some(session) = CurrentSessionSelector.select(state) else {
            f.render_widget(block, area);
            return;
        };

        let max_frame_count =
            ((area.width - ledgend_width - 2) / (BAR_WIDTH * 2 + GROUP_GAP)) as usize;
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
                let target_ms_per_frame = 1000 / session.display_refresh_rate as u128;
                let is_ui_janky = frame.build.as_millis() > target_ms_per_frame;
                let is_raster_janky = frame.raster.as_millis() > target_ms_per_frame;
                let ui_bar = Bar::default()
                    .value(frame.build.as_millis() as u64)
                    .text_value("".to_string())
                    .style(Style::default().fg(if is_ui_janky { UI_JANK_COLOR } else { UI_COLOR }));
                let raster_bar = Bar::default()
                    .value(frame.raster.as_millis() as u64)
                    .text_value("".to_string())
                    .style(Style::default().fg(if is_raster_janky {
                        RASTER_JANK_COLOR
                    } else {
                        RASTER_COLOR
                    }));
                BarGroup::default().bars(&[ui_bar, raster_bar]).label(
                    Line::from(if index % 2 == 1 {
                        frame.number.to_string()
                    } else {
                        " ".to_string()
                    })
                    .centered(),
                )
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

        let ledgend = Paragraph::new(vec![
            { Line::from(Span::styled("UI", Style::default().fg(UI_COLOR))) },
            Line::from(Span::styled("Raster", Style::default().fg(RASTER_COLOR))),
            Line::from(vec![Span::styled(
                "UI Jank",
                Style::default().fg(UI_JANK_COLOR),
            )]),
            Line::from(vec![Span::styled(
                "Raster Jank",
                Style::default().fg(RASTER_JANK_COLOR),
            )]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        );

        f.render_widget(barchart, area);
        f.render_widget(Clear, ledgend_area);
        f.render_widget(ledgend, ledgend_area);
    }
}
