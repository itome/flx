use uuid::Uuid;

use crate::daemon::run::FlutterRun;

pub struct Session {
    pub id: String,
    pub run: FlutterRun,
}

impl Session {
    pub fn new(
        project_root: Option<String>,
        device_id: Option<String>,
        flavor: Option<String>,
    ) -> Self {
        let run = FlutterRun::new(project_root, device_id, flavor).unwrap();
        Self {
            id: Uuid::new_v4().to_string(),
            run,
        }
    }
}
