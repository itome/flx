use crate::daemon::run::FlutterRun;

pub struct Session {
    pub run: FlutterRun,
}

impl Session {
    pub fn new(project_root: Option<&str>, flavor: Option<&str>) -> Self {
        let run = FlutterRun::new(project_root, flavor).unwrap();
        Self { run }
    }
}
