use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::TuiAction;
use crate::redux::action::Action;
use crate::redux::selector::availale_devices::AvailableDevicesSelector;
use crate::redux::state::{State, Tab};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::{daemon::flutter::FlutterDaemon, tui::Frame};
use color_eyre::eyre::{eyre, Result};

use super::Component;

#[derive(Default)]
pub struct SelectFlavorPopupComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl SelectFlavorPopupComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextFlavorForRunning.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousFlavorForRunning.into())?;
        Ok(())
    }

    fn run_new_app(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(ThunkAction::RunNewApp.into())?;
        self.hide_popup()?;
        Ok(())
    }

    fn hide_popup(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::HideSelectFlavorPopUp.into())?;
        Ok(())
    }

    fn hide_device_popup(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::HideSelectDevicePopUp.into())?;
        Ok(())
    }
}

impl Component for SelectFlavorPopupComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent, state: &State) -> Result<()> {
        if !state.select_flavor_popup.visible {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            KeyCode::Enter => {
                self.run_new_app()?;
                self.hide_device_popup()?;
            }
            KeyCode::Esc => self.hide_popup()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        let flavors = &state.flavors;

        let items = flavors
            .iter()
            .map(|flavor| {
                let item = ListItem::new(format!(" {} ", flavor.clone()));
                if state.select_flavor_popup.selected_flavor == Some(flavor.clone()) {
                    item.add_modifier(Modifier::REVERSED)
                        .add_modifier(Modifier::BOLD)
                } else {
                    item
                }
            })
            .collect::<Vec<_>>();

        let block = Block::default()
            .title("Which flavor do you want to use?")
            .borders(Borders::ALL)
            .border_style(Style::default());

        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}
