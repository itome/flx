use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::TuiAction,
    redux::{
        action::Action,
        state::{self, Focus, State},
        ActionOrThunk,
    },
};

#[derive(Default)]
pub struct SelectTabControllerComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl SelectTabControllerComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next_home_tab(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::NextHomeTab.into())?;
        Ok(())
    }

    fn previous_home_tab(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::PreviousHomeTab.into())?;
        Ok(())
    }

    fn next_devtools_tab(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::NextDevToolsTab.into())?;
        Ok(())
    }

    fn previous_devtools_tab(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::PreviousDevToolsTab.into())?;
        Ok(())
    }

    fn exit_devtools(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::ExitDevTools.into())?;
        Ok(())
    }
}

impl Component for SelectTabControllerComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent, state: &State) -> Result<()> {
        if state.popup.is_some() {
            return Ok(());
        }
        match state.focus {
            Focus::Home(_) => match key.code {
                KeyCode::Left | KeyCode::Char('h') => self.previous_home_tab()?,
                KeyCode::Right | KeyCode::Char('l') => self.next_home_tab()?,
                _ => {}
            },
            Focus::DevTools(_) => match key.code {
                KeyCode::Left | KeyCode::Char('h') => self.previous_devtools_tab()?,
                KeyCode::Right | KeyCode::Char('l') => self.next_devtools_tab()?,
                KeyCode::Esc => self.exit_devtools()?,
                _ => {}
            },
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {}
}
