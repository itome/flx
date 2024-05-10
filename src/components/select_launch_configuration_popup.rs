use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Rect;
use ratatui::{prelude::*, widgets::*};
use redux_rs::Selector;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::TuiAction;
use crate::redux::action::Action;
use crate::redux::selector::selected_device::selected_device_selector;
use crate::redux::state::{Home, PopUp, State};
use crate::redux::thunk::ThunkAction;
use crate::redux::ActionOrThunk;
use crate::tui::Frame;
use color_eyre::eyre::{eyre, Result};
use daemon::flutter::FlutterDaemon;

use super::Component;

#[derive(Default)]
pub struct SelectLaunchConfigurationPopupComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl SelectLaunchConfigurationPopupComponent {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::NextLaunchConfiguration.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::PreviousLaunchConfiguration.into())?;
        Ok(())
    }

    fn show_select_device_popup(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::ShowSelectDevicePopUp.into())?;
        Ok(())
    }

    fn hide_popup(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .ok_or_else(|| eyre!("action_tx is None"))?
            .send(Action::HideSelectLaunchConfigurationPopuup.into())?;
        Ok(())
    }
}

impl Component for SelectLaunchConfigurationPopupComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, state: &State) -> Result<()> {
        if state.popup != Some(PopUp::SelectLaunchConfiguration) {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous()?,
            KeyCode::Down | KeyCode::Char('j') => self.next()?,
            KeyCode::Enter => {
                self.hide_popup()?;
                self.show_select_device_popup()?;
            }
            KeyCode::Esc => self.hide_popup()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {
        if state.popup != Some(PopUp::SelectLaunchConfiguration) {
            return;
        }

        f.render_widget(Clear, area);

        let items = state
            .launch_configurations
            .iter()
            .enumerate()
            .map(|(index, config)| {
                let item = ListItem::new(format!(" {} ", config.name.clone()));
                if state.select_launch_configuration_poopup.selected_index == Some(index) {
                    item.add_modifier(Modifier::REVERSED)
                        .add_modifier(Modifier::BOLD)
                } else {
                    item
                }
            })
            .collect::<Vec<_>>();

        let block = Block::default()
            .title("Select launch configuration")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green));

        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}
