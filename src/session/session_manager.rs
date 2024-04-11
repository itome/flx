use std::sync::Arc;

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
    pub fn new(
        project_root: Option<String>,
        device_id: Option<String>,
        program: Option<String>,
        flutter_mode: Option<String>,
        cwd: Option<String>,
        args: Option<Vec<String>>,
        use_fvm: bool,
    ) -> Self {
        let run = FlutterRun::new(
            project_root,
            device_id,
            program,
            flutter_mode,
            cwd,
            args,
            use_fvm,
        )
        .unwrap();
        let vm_service = VmService::new();
        Self {
            id: Uuid::new_v4().to_string(),
            run,
            vm_service,
        }
    }
}

pub struct SessionManager {
    project_root: Option<String>,
    pub session0: Arc<RwLock<Option<Session>>>,
    pub session1: Arc<RwLock<Option<Session>>>,
    pub session2: Arc<RwLock<Option<Session>>>,
    pub session3: Arc<RwLock<Option<Session>>>,
    pub session4: Arc<RwLock<Option<Session>>>,
    pub session5: Arc<RwLock<Option<Session>>>,
    pub session6: Arc<RwLock<Option<Session>>>,
    pub session7: Arc<RwLock<Option<Session>>>,
    pub session8: Arc<RwLock<Option<Session>>>,
    pub session9: Arc<RwLock<Option<Session>>>,
}

impl SessionManager {
    pub fn new(project_root: Option<String>) -> Self {
        Self {
            project_root,
            session0: Arc::new(RwLock::new(None)),
            session1: Arc::new(RwLock::new(None)),
            session2: Arc::new(RwLock::new(None)),
            session3: Arc::new(RwLock::new(None)),
            session4: Arc::new(RwLock::new(None)),
            session5: Arc::new(RwLock::new(None)),
            session6: Arc::new(RwLock::new(None)),
            session7: Arc::new(RwLock::new(None)),
            session8: Arc::new(RwLock::new(None)),
            session9: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn run_new_app(
        &self,
        device_id: Option<String>,
        program: Option<String>,
        flutter_mode: Option<String>,
        cwd: Option<String>,
        args: Option<Vec<String>>,
        use_fvm: bool,
    ) -> Result<String> {
        let session = Session::new(
            self.project_root.clone(),
            device_id,
            program,
            flutter_mode,
            cwd,
            args,
            use_fvm,
        );
        let session_id = session.id.clone();
        for slot in self.sessions() {
            if slot.read().await.is_none() {
                *slot.write().await = Some(session);
                return Ok(session_id);
            }
        }
        Err(eyre!("No available session slots"))
    }

    pub async fn session(&self, id: String) -> Result<Arc<RwLock<Option<Session>>>> {
        for s in self.sessions() {
            if let Some(session) = s.read().await.as_ref() {
                if session.id == id {
                    return Ok(s.clone());
                }
            }
        }
        Err(eyre!("Stdout is not available"))
    }

    fn sessions(&self) -> Vec<Arc<RwLock<Option<Session>>>> {
        vec![
            self.session0.clone(),
            self.session1.clone(),
            self.session2.clone(),
            self.session3.clone(),
            self.session4.clone(),
            self.session5.clone(),
            self.session6.clone(),
            self.session7.clone(),
            self.session8.clone(),
            self.session9.clone(),
        ]
    }
}
