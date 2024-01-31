use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use ratatui::prelude::*;
use redux_rs::middlewares::thunk::thunk;
use redux_rs::Store;
use redux_rs::{
    middlewares::thunk::{self, ThunkMiddleware},
    StoreApi,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::components;
use crate::components::devices::DevicesComponent;
use crate::components::project::ProjectComponent;
use crate::components::runners::RunnersComponent;
use crate::components::select_tab_handler::SelectTabControllerComponent;
use crate::daemon::flutter::FlutterDaemon;
use crate::redux::state::State;
use crate::redux::thunk::context::Context;
use crate::redux::thunk::watch_devices::WatchDevicesThunk;
use crate::redux::thunk::{thunk_impl, ThunkAction};
use crate::{
    action::TuiAction,
    components::Component,
    config::Config,
    mode::Mode,
    redux::{reducer::reducer, ActionOrThunk},
    tui::{self, Tui},
};

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let config = Config::new()?;
        let mode = Mode::Home;
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(ProjectComponent::new()),
                Box::new(RunnersComponent::new()),
                Box::new(DevicesComponent::new()),
                Box::new(SelectTabControllerComponent::new()),
            ],
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let store = Store::new(reducer).wrap(ThunkMiddleware).await;
        let daemon = Arc::new(FlutterDaemon::new()?);
        let context = Arc::new(Context::new(daemon.clone()));
        let (tui_action_tx, mut tui_action_rx) = mpsc::unbounded_channel::<TuiAction>();
        let (redux_action_tx, mut redux_action_rx) = mpsc::unbounded_channel::<ActionOrThunk>();

        redux_action_tx.send(ThunkAction::WatchDevices.into())?;

        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(redux_action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => tui_action_tx.send(TuiAction::Quit)?,
                    tui::Event::Tick => tui_action_tx.send(TuiAction::Tick)?,
                    tui::Event::Render => tui_action_tx.send(TuiAction::Render)?,
                    tui::Event::Resize(x, y) => tui_action_tx.send(TuiAction::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                tui_action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    tui_action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        tui_action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = tui_action_rx.try_recv() {
                if action != TuiAction::Tick && action != TuiAction::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    TuiAction::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    TuiAction::Quit => self.should_quit = true,
                    TuiAction::Suspend => self.should_suspend = true,
                    TuiAction::Resume => self.should_suspend = false,
                    TuiAction::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        let state = store.state_cloned().await;
                        self.draw(&mut tui, &state)?;
                    }
                    TuiAction::Render => {
                        let state = store.state_cloned().await;
                        self.draw(&mut tui, &state)?;
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        tui_action_tx.send(action)?
                    };
                }
            }
            while let Ok(action) = redux_action_rx.try_recv() {
                match action {
                    ActionOrThunk::Action(action) => {
                        store.dispatch(action).await;
                    }
                    ActionOrThunk::Thunk(action) => {
                        store
                            .dispatch(thunk::ActionOrThunk::Thunk(Box::new(thunk_impl(
                                action,
                                context.clone(),
                            ))))
                            .await;
                    }
                }
            }

            if self.should_suspend {
                tui.suspend()?;
                tui_action_tx.send(TuiAction::Resume)?;
                tui = tui::Tui::new()?
                    .tick_rate(self.tick_rate)
                    .frame_rate(self.frame_rate);
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn draw(&mut self, tui: &mut Tui, state: &State) -> Result<()> {
        tui.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
                .split(f.size());
            let tab_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(layout[0]);

            self.components[0].draw(f, tab_layout[0], state);
            self.components[1].draw(f, tab_layout[1], state);
            self.components[2].draw(f, tab_layout[2], state);
        })?;
        Ok(())
    }
}
