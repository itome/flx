use std::fs;
use std::sync::Arc;

use color_eyre::owo_colors::OwoColorize;
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{Focus, State, Tab};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect_tui::into_span;

use super::Component;

#[derive(Default)]
pub struct PubspecComponent {
    pub pubspec_path: String,
    pub lines: Vec<Vec<(String, Style)>>,
}

impl PubspecComponent {
    pub fn new(pubspec_path: String) -> Self {
        Self {
            pubspec_path,
            lines: vec![],
        }
    }
}

impl Component for PubspecComponent {
    fn init(&mut self, area: Rect) -> Result<()> {
        let pubspec_content = fs::read_to_string(&self.pubspec_path)?;
        self.lines = vec![];
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_by_extension("yaml").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (dark)"]);
        for (line_num, line) in LinesWithEndings::from(&pubspec_content).enumerate() {
            let line_styles: Vec<(String, Style)> = h
                .highlight_line(line, &ps)?
                .into_iter()
                .map(|segment| {
                    (
                        segment.1.to_string(),
                        syntect_tui::translate_style(segment.0)
                            .ok()
                            .unwrap_or_default()
                            .underline_color(Color::Reset)
                            .bg(Color::Reset),
                    )
                })
                .collect();
            self.lines.push(line_styles);
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let lines = self.lines.iter().map(|items| {
            return Line::from(
                items
                    .iter()
                    .map(|item| {
                        return Span::styled(&item.0, item.1.bg(Color::Reset));
                    })
                    .collect::<Vec<_>>(),
            );
        });
        let block = Block::default()
            .title("pubspec.yaml")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        let text = List::new(lines)
            .block(block)
            .highlight_symbol(symbols::scrollbar::HORIZONTAL.end)
            .highlight_spacing(HighlightSpacing::Always);
        f.render_widget(text, area);
    }
}
