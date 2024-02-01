use std::{collections::HashMap, sync::Arc};

use super::session::Session;
use crate::daemon::flutter::FlutterDaemon;
use color_eyre::eyre::Result;
use uuid::Uuid;

pub struct SessionManager {
    project_root: Option<String>,
    pub sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new(project_root: Option<String>) -> Self {
        Self {
            sessions: HashMap::new(),
            project_root,
        }
    }

    pub fn run_new_app(&mut self) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let app = Session::new(
            self.project_root.as_ref().map(|path| -> &str { &path }),
            None,
        );
        self.sessions.insert(id.clone(), app);
        Ok(id)
    }
}
