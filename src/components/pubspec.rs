use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{Focus, Home, State};
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
    pub project_root: PathBuf,
    pub lines: Vec<Vec<(String, Style)>>,
    state: ListState,
    scroll_poition: usize,
}

impl PubspecComponent {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            lines: vec![],
            scroll_poition: 0,
            state: ListState::default(),
        }
    }
}

impl Component for PubspecComponent {
    fn init(&mut self, area: Rect) -> Result<()> {
        let pubspec_content = fs::read_to_string(self.project_root.join("pubspec.yaml"))?;
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

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.focus != Focus::Home(Home::Project) || state.popup.is_some() {
            return Ok(());
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_poition = if self.scroll_poition > 0 {
                    self.scroll_poition - 1
                } else {
                    0
                };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll_poition = if self.scroll_poition < self.lines.len() - 1 {
                    self.scroll_poition + 1
                } else {
                    self.lines.len() - 1
                };
            }
            _ => {}
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

        self.state.select(Some(self.scroll_poition));
        let mut scrollbar_state = ScrollbarState::new(lines.len()).position(self.scroll_poition);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let block = Block::default()
            .title("pubspec.yaml")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::White));
        let text = List::new(lines)
            .block(block)
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_spacing(HighlightSpacing::Never);

        f.render_stateful_widget(text, area, &mut self.state);
        f.render_stateful_widget(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}
