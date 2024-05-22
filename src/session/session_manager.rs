use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use color_eyre::eyre::{eyre, Result};
use daemon::run::FlutterRun;
use devtools::vm_service::VmService;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct Session {
    pub id: String,
    pub run: FlutterRun,
    pub vm_service: VmService,
}

impl Session {
    pub fn new(run: FlutterRun, vm_service: VmService) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            run,
            vm_service,
        }
    }
}

pub struct SessionManager {
    sessions: RwLock<HashMap<String, Arc<Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub async fn run_new_app(
        &self,
        project_root: PathBuf,
        device_id: Option<String>,
        program: Option<String>,
        flutter_mode: Option<String>,
        cwd: Option<String>,
        args: Option<Vec<String>>,
        use_fvm: bool,
    ) -> Result<String> {
        log::info!("Running new app");
        let run = FlutterRun::new(
            project_root.clone(),
            device_id.clone(),
            program.clone(),
            flutter_mode.clone(),
            cwd.clone(),
            args.clone(),
            use_fvm,
        )?;
        let vm_service = VmService::new();
        let session = Session::new(run, vm_service);
        let session_id = session.id.clone();
        self.sessions
            .write()
            .await
            .insert(session_id.clone(), Arc::new(session));
        return Ok(session_id);
    }

    pub async fn session(&self, id: String) -> Option<Arc<Session>> {
        self.sessions.read().await.get(&id).cloned()
    }

    pub async fn remove_session(&self, id: String) -> Result<()> {
        if self.sessions.write().await.remove(&id).is_none() {
            return Err(eyre!("Session not found"));
        }
        Ok(())
    }
}
