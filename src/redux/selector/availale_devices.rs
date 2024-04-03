use redux_rs::Selector;

use crate::redux::state::State;
use daemon::io::device::Device;

pub fn available_devices_selector(state: &State) -> impl Iterator<Item = &Device> {
    state.devices.iter().filter(|d| {
        state.supported_platforms.contains(&d.platform_type)
            && state.sessions.iter().all(|s| {
                if let Some(device_id) = &s.device_id {
                    device_id != &d.id
                } else {
                    false
                }
            })
    })
}
