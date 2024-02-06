use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::TuiAction,
    config::{Config, KeyBindings},
    redux::{action::Action, state::State, ActionOrThunk},
};

#[derive(Default)]
pub struct SelectTabControllerComponent {
    action_tx: Option<UnboundedSender<ActionOrThunk>>,
}

impl SelectTabControllerComponent {
    pub fn new() -> Self {
        Self::default()
    }

    fn next(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::NextTab.into())?;
        Ok(())
    }

    fn previous(&self) -> Result<()> {
        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::PreviousTab.into())?;
        Ok(())
    }
}

impl Component for SelectTabControllerComponent {
    fn register_action_handler(&mut self, tx: UnboundedSender<ActionOrThunk>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent, _: &State) -> Result<()> {
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => self.previous()?,
            KeyCode::Right | KeyCode::Char('l') => self.next()?,
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect, state: &State) {}
}
