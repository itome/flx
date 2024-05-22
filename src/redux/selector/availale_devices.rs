use redux_rs::Selector;

use crate::redux::state::State;
use daemon::io::device::Device;

pub fn available_devices_selector(state: &State) -> impl Iterator<Item = &Device> {
    let mut project_root = state.project_root.clone();

    if !state.launch_configurations.is_empty() {
        if let Some(selected_index) = state.select_launch_configuration_poopup.selected_index {
            let selected_launch_configuration = &state.launch_configurations[selected_index];
            if let Some(cwd) = &selected_launch_configuration.cwd {
                project_root = project_root.clone().join(cwd);
            }
        };
    }

    state.devices.iter().filter(move |d| {
        let Some(supported_platforms) = state.supported_platforms.get(&project_root) else {
            return false;
        };

        supported_platforms.contains(&d.platform_type)
            && state.sessions.iter().all(|s| {
                if let Some(device_id) = &s.device_id {
                    device_id != &d.id || s.stopped
                } else {
                    false
                }
            })
    })
}
