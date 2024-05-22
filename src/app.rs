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
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::{Mutex, RwLock};

use crate::components;
use crate::components::app::AppComponent;
use crate::components::device::DeviceComponent;
use crate::components::devices::DevicesComponent;
use crate::components::frame_analysis::FrameAnalysisComponent;
use crate::components::frames::FramesComponent;
use crate::components::inspector::InspectorComponent;
use crate::components::launch_configurations::LaunchConfigurationsComponent;
use crate::components::logs::LogsComponent;
use crate::components::network::NetworkComponent;
use crate::components::network_request::NetworkRequestComponent;
use crate::components::performance::PerformanceComponent;
use crate::components::project::ProjectComponent;
use crate::components::pubspec::PubspecComponent;
use crate::components::runners::RunnersComponent;
use crate::components::sdk_version::SdkVersionComponent;
use crate::components::select_device_popup::SelectDevicePopupComponent;
use crate::components::select_launch_configuration_popup::SelectLaunchConfigurationPopupComponent;
use crate::components::select_tab_handler::SelectTabControllerComponent;
use crate::components::widget_details::WidgetDetailsComponent;
use crate::redux::action::Action;
use crate::redux::selector::current_session::current_session_selector;
use crate::redux::state::{
    DevTools, Focus, Home, SelectDevicePopupState, SelectLaunchConfigurationPopupState, State,
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
    Device,
    SelectDevicePopup,
    Frames,
    FrameAnalysis,
    Logs,
    Network,
    NetworkRequest,
    SelectTabController,
    SelectFlavorPopup,
    Pubspec,
    App,
    Performance,
    Inspector,
    WidgetDetails,
    LaunchConfigurations,
    SdkVersion,
}

pub struct App {
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub project_root: PathBuf,
    pub use_fvm: bool,
    pub components: HashMap<ComponentId, Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
}

impl App {
    pub fn new(project_root: Option<String>, use_fvm: bool) -> Result<Self> {
        let project_root = if let Some(project_root) = project_root.clone() {
            let path = Path::new(&project_root).to_path_buf();
            if !path.exists() {
                return Err(eyre!("Invalid project root"));
            }
            path
        } else {
            env::current_dir()?
        };

        Ok(Self {
            tick_rate: 4.0,
            frame_rate: 60.0,
            project_root: project_root.clone(),
            use_fvm,
            components: HashMap::from([
                (
                    ComponentId::Project,
                    Box::new(ProjectComponent::new(project_root.clone())) as Box<dyn Component>,
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
                    ComponentId::Device,
                    Box::new(DeviceComponent::new()) as Box<dyn Component>,
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
                    Box::new(SelectLaunchConfigurationPopupComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::Pubspec,
                    Box::new(PubspecComponent::new(project_root)) as Box<dyn Component>,
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
                    ComponentId::WidgetDetails,
                    Box::new(WidgetDetailsComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::NetworkRequest,
                    Box::new(NetworkRequestComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::FrameAnalysis,
                    Box::new(FrameAnalysisComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::LaunchConfigurations,
                    Box::new(LaunchConfigurationsComponent::new()) as Box<dyn Component>,
                ),
                (
                    ComponentId::SdkVersion,
                    Box::new(SdkVersionComponent::new()) as Box<dyn Component>,
                ),
            ]),
            should_quit: false,
            should_suspend: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let initial_state = State::new(self.project_root.clone());
        let store = Store::new_with_state(reducer, initial_state)
            .wrap(ThunkMiddleware)
            .await;
        let daemon = Arc::new(FlutterDaemon::new(self.use_fvm)?);
        let session_manager = Arc::new(SessionManager::new());
        let context = Arc::new(Context::new(daemon.clone(), session_manager.clone()));
        let (tui_action_tx, mut tui_action_rx) = mpsc::unbounded_channel::<TuiAction>();
        let (redux_action_tx, mut redux_action_rx) = mpsc::unbounded_channel::<ActionOrThunk>();

        redux_action_tx.send(ThunkAction::WatchDevices.into())?;
        redux_action_tx.send(ThunkAction::LoadEmulators.into())?;
        redux_action_tx.send(ThunkAction::LoadVSCodeLaunchSetting.into())?;
        redux_action_tx.send(
            ThunkAction::LoadSdkVersions {
                use_fvm: self.use_fvm,
            }
            .into(),
        )?;

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

    fn component(&mut self, id: &ComponentId) -> &mut Box<dyn Component> {
        self.components.get_mut(id).unwrap()
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

            self.component(&ComponentId::Project)
                .draw(f, tab_layout[0], state);
            self.component(&ComponentId::Runners)
                .draw(f, tab_layout[1], state);
            self.component(&ComponentId::Devices)
                .draw(f, tab_layout[2], state);

            if state.focus == Focus::Home(Home::Runners) {
                if let Some(session) = current_session_selector(state) {
                    if !session.started {
                        self.component(&ComponentId::Logs).draw(f, layout[1], state);
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
                        self.component(&ComponentId::Frames)
                            .draw(f, vertical_layout[0], state);
                        self.component(&ComponentId::Inspector).draw(
                            f,
                            horizontal_layout[0],
                            state,
                        );
                        self.component(&ComponentId::Network)
                            .draw(f, horizontal_layout[1], state);
                        self.component(&ComponentId::Logs)
                            .draw(f, horizontal_layout[2], state);
                    }
                } else {
                    let horizontal_layout = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
                        .split(layout[1]);
                    self.component(&ComponentId::LaunchConfigurations).draw(
                        f,
                        horizontal_layout[0],
                        state,
                    );
                    self.component(&ComponentId::SdkVersion)
                        .draw(f, horizontal_layout[1], state);
                }
            }

            if state.focus == Focus::Home(Home::Project) {
                self.component(&ComponentId::Pubspec)
                    .draw(f, layout[1], state);
            }

            if state.focus == Focus::Home(Home::Devices) {
                self.component(&ComponentId::Device)
                    .draw(f, layout[1], state);
            }

            let popup_area = centered_rect(60, 20, f.size());
            self.component(&ComponentId::SelectDevicePopup)
                .draw(f, popup_area, state);

            let popup_area = centered_rect(60, 40, f.size());
            self.component(&ComponentId::SelectFlavorPopup)
                .draw(f, popup_area, state);
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
                    Focus::DevTools(DevTools::Inspector)
                    | Focus::DevTools(DevTools::WidgetDetails) => vec![
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

            self.component(&ComponentId::App)
                .draw(f, tab_layout[0], state);
            self.component(&ComponentId::Inspector)
                .draw(f, tab_layout[1], state);
            self.component(&ComponentId::Performance)
                .draw(f, tab_layout[2], state);
            self.component(&ComponentId::Network)
                .draw(f, tab_layout[3], state);

            match state.focus {
                Focus::DevTools(DevTools::Performance) => {
                    let vertical_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                        .split(layout[1]);
                    self.component(&ComponentId::Frames)
                        .draw(f, vertical_layout[0], state);
                    self.component(&ComponentId::FrameAnalysis)
                        .draw(f, vertical_layout[1], state)
                }
                Focus::DevTools(DevTools::App) => {
                    self.component(&ComponentId::Logs).draw(f, layout[1], state);
                }
                Focus::DevTools(DevTools::Network) | Focus::DevTools(DevTools::NetworkRequest) => {
                    self.component(&ComponentId::NetworkRequest)
                        .draw(f, layout[1], state);
                }
                Focus::DevTools(DevTools::Inspector) | Focus::DevTools(DevTools::WidgetDetails) => {
                    self.component(&ComponentId::WidgetDetails)
                        .draw(f, layout[1], state);
                }
                _ => {}
            }
        })?;
        Ok(())
    }
}
