use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::TuiAction;
use crate::redux::action::Action;
use crate::redux::selector::availale_devices::available_devices_selector;
use crate::redux::selector::selected_device::selected_device_selector;
use crate::redux::state::{Home, PopUp, State};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct SelectDevicePopupComponent {
    use_fvm: bool,
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl SelectDevicePopupComponent {
    pub fn new(use_fvm: bool) -> Self {
        Self {
            use_fvm,
            ..Self::default()
        }
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextDeviceForRunning.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousDeviceForRunning.into())?;
        Ok(())
    }

    fn run_new_app(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(
                ThunkAction::RunNewApp {
                    use_fvm: self.use_fvm,
                }
                .into(),
            )?;
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

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.popup != Some(PopUp::SelectDevice) {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            KeyCode::Enter => {
                self.hide_popup()?;
                self.run_new_app()?;
            }
            KeyCode::Esc => self.hide_popup()?,

            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        if state.popup != Some(PopUp::SelectDevice) {
            return;
        }

        f.render_widget(Clear, area);

        let devices = available_devices_selector(state);

        let items = devices
            .map(|device| {
                let item = ListItem::new(format!(" {} ", device.name.clone()));
                if let Some(selected_device_id) = &state.select_device_popup.selected_device_id {
                    if selected_device_id == &device.id {
                        return item
                            .add_modifier(Modifier::REVERSED)
                            .add_modifier(Modifier::BOLD);
                    }
                }
                item
            })
            .collect::<Vec<_>>();

        let block = Block::default()
            .title("Which device do you want to use?")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green));

        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}
