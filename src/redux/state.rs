use crate::daemon::io::device::Device;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct State {
    pub devices: Vec<Device>,
}
