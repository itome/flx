use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::TuiAction;
use crate::redux::action::Action;
use crate::redux::state::{State, Tab};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::{eyre, Result};

use super::Component;

#[derive(Default)]
pub struct SelectDevicePopupComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl SelectDevicePopupComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn run_new_app(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::RunNewApp.into())?;
        Ok(())
    }

    fn hide_popup(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::HideSelectDevicePopUp.into())?;
        Ok(())
    }
}

impl Component for SelectDevicePopupComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<TuiAction>> {
        if key.code == KeyCode::Esc {
            self.hide_popup()?;
        } else if key.code == KeyCode::Enter {
            self.run_new_app()?;
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        log::info!(
            "Drawing SelectDevicePopupComponent with devices: {:?}",
            state.devices
        );
        let devices = state.devices.iter().filter(|d| {
            state.supported_platforms.contains(&d.platform_type)
                && state
                    .sessions
                    .iter()
                    .all(|s| s.device_id != Some(d.id.clone()))
        });

        let items = devices
            .map(|device| ListItem::new(device.name.clone()).style(Style::default()))
            .collect::<Vec<_>>();

        let block = Block::default()
            .title("Which device do you want to use?")
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL)
            .border_style(Style::default());

        let list = List::new(items)
            .block(block)
            .fg(Color::White)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

        f.render_widget(list, area);
    }
}
