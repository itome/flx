use uuid::Uuid;

use crate::daemon::run::FlutterRun;

pub struct Session {
    pub id: String,
    pub run: FlutterRun,
}

impl Session {
    pub fn new(project_root: Option<&str>, flavor: Option<&str>) -> Self {
        let run = FlutterRun::new(project_root, flavor).unwrap();
        Self {
            id: Uuid::new_v4().to_string(),
            run,
        }
    }
}
