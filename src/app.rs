use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use ratatui::prelude::*;
use ratatui::widgets::Clear;
use redux_rs::middlewares::thunk::thunk;
use redux_rs::{
    middlewares::thunk::{self, ThunkMiddleware},
    StoreApi,
};
use redux_rs::{Selector, Store};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::{Mutex, RwLock};

use crate::components;
use crate::components::devices::DevicesComponent;
use crate::components::frames::FramesComponent;
use crate::components::logs::LogsComponent;
use crate::components::network::NetworkComponent;
use crate::components::project::ProjectComponent;
use crate::components::pubspec::PubspecComponent;
use crate::components::runners::RunnersComponent;
use crate::components::select_device_popup::SelectDevicePopupComponent;
use crate::components::select_flavor_popup::SelectFlavorPopupComponent;
use crate::components::select_tab_handler::SelectTabControllerComponent;
use crate::redux::action::Action;
use crate::redux::selector::current_session::CurrentSessionSelector;
use crate::redux::state::{Focus, SelectDevicePopupState, SelectFlavorPopupState, State, Tab};
use crate::redux::thunk::context::Context;
use crate::redux::thunk::watch_devices::WatchDevicesThunk;
use crate::redux::thunk::{thunk_impl, ThunkAction};
use crate::session::session_manager::SessionManager;
use crate::utils::centered_rect;
use crate::{
    action::TuiAction,
    components::Component,
    config::Config,
    mode::Mode,
    redux::{reducer::reducer, ActionOrThunk},
    tui::{self, Tui},
};
use daemon::flutter::FlutterDaemon;

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub project_root: Option<String>,
    pub use_fvm: bool,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(project_root: Option<String>, use_fvm: bool) -> Result<Self> {
        let config = Config::new()?;
        let mode = Mode::Home;
        let pubspec_path = project_root.clone().unwrap_or(".".to_string()) + "/pubspec.yaml";
        Ok(Self {
            tick_rate: 4.0,
            frame_rate: 60.0,
            project_root,
            use_fvm,
            components: vec![
                Box::new(ProjectComponent::new()),
                Box::new(RunnersComponent::new()),
                Box::new(DevicesComponent::new()),
                Box::new(SelectDevicePopupComponent::new(use_fvm)),
                Box::new(FramesComponent::new()),
                Box::new(LogsComponent::new()),
                Box::new(NetworkComponent::new()),
                Box::new(SelectTabControllerComponent::new()),
                Box::new(SelectFlavorPopupComponent::new(use_fvm)),
                Box::new(PubspecComponent::new(pubspec_path)),
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
        let daemon = Arc::new(FlutterDaemon::new(self.use_fvm)?);
        let session_manager = Arc::new(SessionManager::new(self.project_root.clone()));
        let context = Arc::new(Context::new(daemon.clone(), session_manager.clone()));
        let (tui_action_tx, mut tui_action_rx) = mpsc::unbounded_channel::<TuiAction>();
        let (redux_action_tx, mut redux_action_rx) = mpsc::unbounded_channel::<ActionOrThunk>();

        redux_action_tx.send(
            Action::SetProjectRoot {
                project_root: self.project_root.clone(),
            }
            .into(),
        )?;
        redux_action_tx.send(ThunkAction::WatchDevices.into())?;
        redux_action_tx.send(ThunkAction::LoadSupportedPlatforms.into())?;

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
            let state = store.state_cloned().await;
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
                    component.handle_events(Some(e.clone()), &state)?;
                }
            }

            while let Ok(action) = tui_action_rx.try_recv() {
                match action {
                    TuiAction::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    TuiAction::Quit => self.should_quit = true,
                    TuiAction::Suspend => self.should_suspend = true,
                    TuiAction::Resume => self.should_suspend = false,
                    TuiAction::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        self.draw(&mut tui, &state)?;
                    }
                    TuiAction::Render => {
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
                            .dispatch(thunk::ActionOrThunk::Thunk(thunk_impl(
                                action,
                                context.clone(),
                            )))
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

            if state.select_device_popup.visible {
                let popup_area = centered_rect(60, 20, f.size());
                f.render_widget(Clear, popup_area);
                self.components[3].draw(f, popup_area, state);
            }

            if state.select_flavor_popup.visible {
                let popup_area = centered_rect(60, 40, f.size());
                f.render_widget(Clear, popup_area);
                self.components[8].draw(f, popup_area, state);
            }

            if state.current_focus == Focus::Tab(Tab::Runners)
                || matches!(state.current_focus, Focus::DevTools(_))
            {
                if let Some(session) = CurrentSessionSelector.select(state) {
                    if !session.started {
                        self.components[5].draw(f, layout[1], state);
                    } else {
                        let vertical_layout = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                            .split(layout[1]);
                        let horizontal_layout = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                            .split(vertical_layout[1]);
                        self.components[4].draw(f, vertical_layout[0], state);
                        self.components[5].draw(f, horizontal_layout[0], state);
                        self.components[6].draw(f, horizontal_layout[1], state);
                    }
                }
            }

            if state.current_focus == Focus::Tab(Tab::Project) {
                self.components[9].draw(f, layout[1], state);
            }
        })?;
        Ok(())
    }
}
