use daemon::io::{device::Device, emulator::Emulator};

use crate::redux::state::State;

use super::current_session::current_session_selector;

pub enum DeviceOrEmulator {
    Device(Device),
    Emulator(Emulator),
}

pub fn device_or_emulators_selector(state: &State) -> Vec<DeviceOrEmulator> {
    let mut devices = state
        .devices
        .iter()
        .map(|d| DeviceOrEmulator::Device(d.clone()))
        .collect::<Vec<_>>();

    for emulator in state.emulators.iter() {
        // If there are already running ios simultor, we don't need to show the ios simulator
        if &emulator.id == "apple_ios_simulator"
            && state
                .devices
                .iter()
                .any(|d| &d.platform_type == "ios" && d.emulator)
        {
            continue;
        }

        // If there are already running android emulator, we don't need to show the android emulator
        if state
            .devices
            .iter()
            .any(|d| d.emulator && d.id == emulator.id)
        {
            continue;
        }

        devices.push(DeviceOrEmulator::Emulator(emulator.clone()));
    }
    return devices;
}
