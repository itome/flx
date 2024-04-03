use daemon::io::device::Device;

use crate::redux::state::State;

pub fn selected_device_selector(state: &State) -> Option<&Device> {
    if let Some(device_id) = &state.select_device_popup.selected_device_id {
        state.devices.iter().find(|d| &d.id == device_id)
    } else {
        None
    }
}
