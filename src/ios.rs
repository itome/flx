use color_eyre::{eyre::eyre, Result};
use std::process::Command;
#[cfg(test)]
mod test;

/// Return schemes if exists.
pub fn get_schemes(project_root: String) -> Result<Option<Vec<String>>> {
    let args = vec!["-list"];
    let output = Command::new("xcodebuild")
        .current_dir(format!("{}/ios", project_root))
        .args(args)
        .output()
        .map_err(|e| eyre!("failed to execute process: {}", e))?;

    let output = String::from_utf8(output.stdout)
        .map_err(|e| eyre!("failed to convert bytes to String: {}", e))?;

    let info = XcodeProjectInfo::new_from_xcode_build_output(&output);

    if info.schemes.is_empty() || info.schemes.contains(&"Runner") {
        return Ok(None);
    }

    Ok(Some(
        info.schemes.into_iter().map(|s| s.to_string()).collect(),
    ))
}

pub struct XcodeProjectInfo<'a> {
    pub targets: Vec<&'a str>,
    pub build_configurations: Vec<&'a str>,
    pub schemes: Vec<&'a str>,
}

impl<'a> XcodeProjectInfo<'_> {
    pub fn new_from_xcode_build_output(output: &'a str) -> XcodeProjectInfo {
        let mut targets = vec![];
        let mut build_configurations = vec![];
        let mut schemes = vec![];
        let mut collector = None;
        for line in output.split("\n") {
            if line.is_empty() {
                collector = None;
                continue;
            } else if line.ends_with("Targets:") {
                collector = Some(&mut targets);
                continue;
            } else if line.ends_with("Build Configurations:") {
                collector = Some(&mut build_configurations);
                continue;
            } else if line.ends_with("Schemes:") {
                collector = Some(&mut schemes);
                continue;
            }
            if let Some(ref mut c) = collector {
                c.push(line.trim());
            }
        }

        XcodeProjectInfo {
            targets: targets,
            build_configurations: build_configurations,
            schemes: schemes,
        }
    }
}
