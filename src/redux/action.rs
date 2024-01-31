use crate::daemon::io::device::Device;

pub enum Action {
    AddDevice { device: Device },
    RemoveDevice { device: Device },

    NextTab,
    PreviousTab,
}
