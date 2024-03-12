use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use crate::session::session_manager::{self, SessionManager};

use daemon::flutter::FlutterDaemon;

pub struct Context {
    pub daemon: Arc<FlutterDaemon>,
    pub session_manager: Arc<SessionManager>,
}

impl Context {
    pub fn new(daemon: Arc<FlutterDaemon>, session_manager: Arc<SessionManager>) -> Self {
        Self {
            daemon,
            session_manager,
        }
    }
}
