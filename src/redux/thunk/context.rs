use std::sync::Arc;

use crate::daemon::flutter::FlutterDaemon;

pub struct Context {
    pub daemon: Arc<FlutterDaemon>,
}

impl Context {
    pub fn new(daemon: Arc<FlutterDaemon>) -> Self {
        Self { daemon }
    }
}
