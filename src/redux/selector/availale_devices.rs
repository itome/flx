use redux_rs::Selector;

use crate::{daemon::io::device::Device, redux::state::State};

pub struct AvailableDevicesSelector;

impl Selector<State> for AvailableDevicesSelector {
    type Result = Vec<Device>;

    fn select(&self, state: &State) -> Self::Result {
        let devices = state.devices.iter().filter(|d| {
            state.supported_platforms.contains(&d.platform_type)
                && state
                    .sessions
                    .iter()
                    .all(|s| s.device_id != Some(d.id.clone()))
        });
        devices.cloned().collect()
    }
}
