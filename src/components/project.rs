use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};

use crate::redux::state::{Focus, Home, State};
use crate::tui::Frame;
use color_eyre::eyre::Result;
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct ProjectComponent {
    pubspec_path: String,
    project_name: Option<String>,
    version: Option<String>,
}

impl ProjectComponent {
    pub fn new(pubspec_path: String) -> Self {
        Self {
            pubspec_path,
            project_name: None,
            version: None,
        }
    }
}

impl Component for ProjectComponent {
    fn init(&mut self, area: Rect) -> Result<()> {
        let file = File::open(&self.pubspec_path)?;
        let reader = BufReader::new(file);
        let pubspec: serde_yaml::Value = serde_yaml::from_reader(reader)?;
        match pubspec {
            serde_yaml::Value::Mapping(map) => {
                self.project_name = map.get("name").map(|v| v.as_str().unwrap().to_string());
                self.version = map.get("version").map(|v| v.as_str().unwrap().to_string());
            }
            _ => {}
        };
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let border_color = if state.focus == Focus::Home(Home::Project) && state.popup.is_none() {
            Color::Green
        } else {
            Color::White
        };

        let block = Block::default()
            .title("Project")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));
        let text = Paragraph::new(format!(
            "{} ({})",
            self.project_name.clone().unwrap_or_default(),
            self.version.clone().unwrap_or_default()
        ))
        .block(block);
        f.render_widget(text, area);
    }
}
