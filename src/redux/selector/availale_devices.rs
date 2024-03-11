use redux_rs::Selector;

use crate::redux::state::State;
use daemon::io::device::Device;

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
