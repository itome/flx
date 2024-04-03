use color_eyre::eyre::{eyre, Result};
use crossterm::event::{KeyCode, KeyEvent};
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
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::{Mutex, RwLock};

use crate::components;
use crate::components::app::AppComponent;
use crate::components::devices::DevicesComponent;
use crate::components::frames::FramesComponent;
use crate::components::inspector::InspectorComponent;
use crate::components::logs::LogsComponent;
use crate::components::network::NetworkComponent;
use crate::components::network_request::NetworkRequestComponent;
use crate::components::performance::PerformanceComponent;
use crate::components::project::ProjectComponent;
use crate::components::pubspec::PubspecComponent;
use crate::components::runners::RunnersComponent;
use crate::components::select_device_popup::SelectDevicePopupComponent;
use crate::components::select_flavor_popup::SelectFlavorPopupComponent;
use crate::components::select_tab_handler::SelectTabControllerComponent;
use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{
    DevTools, Focus, Home, SelectDevicePopupState, SelectFlavorPopupState, State,
};
use crate::redux::thunk::context::Context;
use crate::redux::thunk::watch_devices::WatchDevicesThunk;
use crate::redux::thunk::{thunk_impl, ThunkAction};
use crate::session::session_manager::SessionManager;
use crate::utils::centered_rect;
use crate::{
    action::TuiAction,
    components::Component,
    redux::{reducer::reducer, ActionOrThunk},
    tui::{self, Tui},
};
use daemon::flutter::FlutterDaemon;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ComponentId {
    Project,
    Runners,
    Devices,
    SelectDevicePopup,
    Frames,
    Logs,
    Network,
    NetworkRequest,
    SelectTabController,
    SelectFlavorPopup,
    Pubspec,
    App,
    Performance,
    Inspector,
}

pub struct App {
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub project_root: Option<String>,
    pub use_fvm: bool,
    pub components: HashMap<ComponentId, Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
}

impl App {
    pub fn new(project_root: Option<String>, use_fvm: bool) -> Result<Self> {
        let pubspec_path = project_root.clone().unwrap_or(".".to_string()) + "/pubspec.yaml";

        Ok(Self {
            tick_rate: 4.0,
            frame_rate: 60.0,
            project_root,
            use_fvm,
            components: HashMap::from([
                (
                    ComponentId::Project,
                    Box::new(ProjectComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Runners,
                    Box::new(RunnersComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Devices,
                    Box::new(DevicesComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::SelectDevicePopup,
                    Box::new(SelectDevicePopupComponent::new(use_fvm)) as Box<dyn Component>,
                ),
                (
                    ComponentId::Frames,
                    Box::new(FramesComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Logs,
                    Box::new(LogsComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Network,
                    Box::new(NetworkComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::SelectTabController,
                    Box::new(SelectTabControllerComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::SelectFlavorPopup,
                    Box::new(SelectFlavorPopupComponent::new(use_fvm)) as Box<dyn Component>,
                ),
                (
                    ComponentId::Pubspec,
                    Box::new(PubspecComponent::new(pubspec_path)) as Box<dyn Component>,
                ),
                (
                    ComponentId::App,
                    Box::new(AppComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Performance,
                    Box::new(PerformanceComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Inspector,
                    Box::new(InspectorComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::NetworkRequest,
                    Box::new(NetworkRequestComponent::new()) as Box<dyn Component>,
                ),
            ]),
            should_quit: false,
            should_suspend: false,
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

        for (_, component) in self.components.iter_mut() {
            component.register_action_handler(redux_action_tx.clone())?;
        }

        for (_, component) in self.components.iter_mut() {
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
                    tui::Event::Key(key) => match key.code {
                        KeyCode::Char('q') => tui_action_tx.send(TuiAction::Quit)?,
                        KeyCode::Char('z') => tui_action_tx.send(TuiAction::Suspend)?,
                        _ => {}
                    },
                    _ => {}
                }
                for (_, component) in self.components.iter_mut() {
                    component.handle_events(&e, &state)?;
                }
            }

            while let Ok(action) = tui_action_rx.try_recv() {
                match action {
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
                for (_, component) in self.components.iter_mut() {
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
        match state.focus {
            Focus::Home(_) => self.draw_home(tui, state),
            Focus::DevTools(_) => self.draw_devtools(tui, state),
        }
    }

    fn draw_home(&mut self, tui: &mut Tui, state: &State) -> Result<()> {
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

            self.components
                .get_mut(&ComponentId::Project)
                .unwrap()
                .draw(f, tab_layout[0], state);
            self.components
                .get_mut(&ComponentId::Runners)
                .unwrap()
                .draw(f, tab_layout[1], state);
            self.components
                .get_mut(&ComponentId::Devices)
                .unwrap()
                .draw(f, tab_layout[2], state);

            let popup_area = centered_rect(60, 20, f.size());
            self.components
                .get_mut(&ComponentId::SelectDevicePopup)
                .unwrap()
                .draw(f, popup_area, state);

            let popup_area = centered_rect(60, 40, f.size());
            self.components
                .get_mut(&ComponentId::SelectFlavorPopup)
                .unwrap()
                .draw(f, popup_area, state);

            if state.focus == Focus::Home(Home::Runners) {
                if let Some(session) = current_session_selector(state) {
                    if !session.started {
                        self.components
                            .get_mut(&ComponentId::Logs)
                            .unwrap()
                            .draw(f, layout[1], state);
                    } else {
                        let vertical_layout = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                            .split(layout[1]);
                        let horizontal_layout = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Fill(1),
                                Constraint::Fill(1),
                                Constraint::Fill(1),
                            ])
                            .split(vertical_layout[1]);
                        self.components.get_mut(&ComponentId::Frames).unwrap().draw(
                            f,
                            vertical_layout[0],
                            state,
                        );
                        self.components
                            .get_mut(&ComponentId::Inspector)
                            .unwrap()
                            .draw(f, horizontal_layout[0], state);
                        self.components
                            .get_mut(&ComponentId::Network)
                            .unwrap()
                            .draw(f, horizontal_layout[1], state);
                        self.components.get_mut(&ComponentId::Logs).unwrap().draw(
                            f,
                            horizontal_layout[2],
                            state,
                        );
                    }
                }
            }

            if state.focus == Focus::Home(Home::Project) {
                self.components
                    .get_mut(&ComponentId::Pubspec)
                    .unwrap()
                    .draw(f, layout[1], state);
            }
        })?;
        Ok(())
    }

    fn draw_devtools(&mut self, tui: &mut Tui, state: &State) -> Result<()> {
        tui.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
                .split(f.size());
            let tab_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(match state.focus {
                    Focus::DevTools(DevTools::Inspector) => vec![
                        Constraint::Length(3),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                        Constraint::Length(2),
                    ],
                    Focus::DevTools(DevTools::Performance) => vec![
                        Constraint::Length(3),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                        Constraint::Length(2),
                    ],
                    Focus::DevTools(DevTools::Network)
                    | Focus::DevTools(DevTools::NetworkRequest) => vec![
                        Constraint::Length(3),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                    ],
                    _ => vec![
                        Constraint::Length(3),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                        Constraint::Fill(1),
                    ],
                })
                .split(layout[0]);

            self.components
                .get_mut(&ComponentId::App)
                .unwrap()
                .draw(f, tab_layout[0], state);
            self.components
                .get_mut(&ComponentId::Inspector)
                .unwrap()
                .draw(f, tab_layout[1], state);
            self.components
                .get_mut(&ComponentId::Performance)
                .unwrap()
                .draw(f, tab_layout[2], state);
            self.components
                .get_mut(&ComponentId::Network)
                .unwrap()
                .draw(f, tab_layout[3], state);

            if state.focus == Focus::DevTools(DevTools::Performance) {
                let vertical_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(layout[1]);
                self.components.get_mut(&ComponentId::Frames).unwrap().draw(
                    f,
                    vertical_layout[0],
                    state,
                );
            } else if state.focus == Focus::DevTools(DevTools::App) {
                self.components
                    .get_mut(&ComponentId::Logs)
                    .unwrap()
                    .draw(f, layout[1], state);
            } else if state.focus == Focus::DevTools(DevTools::Network)
                || state.focus == Focus::DevTools(DevTools::NetworkRequest)
            {
                self.components
                    .get_mut(&ComponentId::NetworkRequest)
                    .unwrap()
                    .draw(f, layout[1], state);
            }
        })?;
        Ok(())
    }
}
