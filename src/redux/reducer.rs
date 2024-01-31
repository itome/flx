use super::{
    action::Action,
    state::{State, Tab},
};

pub fn reducer(state: State, action: Action) -> State {
    match action {
        Action::AddDevice { device } => State {
            devices: [state.devices, vec![device]].concat(),
            ..state
        },
        Action::RemoveDevice { device } => State {
            devices: state.devices.into_iter().filter(|d| d != &device).collect(),
            ..state
        },
        Action::NextTab => State {
            selected_tab: match state.selected_tab {
                Tab::Project => Tab::Runners,
                Tab::Runners => Tab::Devices,
                Tab::Devices => Tab::Project,
            },
            ..state
        },
        Action::PreviousTab => State {
            selected_tab: match state.selected_tab {
                Tab::Project => Tab::Devices,
                Tab::Runners => Tab::Project,
                Tab::Devices => Tab::Runners,
            },
            ..state
        },
    }
}
