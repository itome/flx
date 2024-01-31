use crate::daemon::io::device::Device;

#[derive(Clone, PartialEq, Eq)]
pub enum Tab {
    Project,
    Runners,
    Devices,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Project
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct State {
    pub devices: Vec<Device>,
    pub selected_tab: Tab,
}
