use color_eyre::{eyre::eyre, Result};
#[cfg(test)]
pub mod test;

use regex::Regex;
use std::{collections::HashSet, env, process::Command};

/// Return schemes if exists.
pub fn get_schemes(project_root: String) -> Result<Option<Vec<String>>> {
    let args = vec!["app:tasks", "--all", "--console=auto"];
    let mut command = Command::new("./gradlew");
    let mut command = command
        .current_dir(format!("{}/android", project_root))
        .args(args);

    let os = env::consts::OS;
    if os == "macos" {
        command = command.env(
            "JAVA_HOME",
            "/Applications/Android Studio.app/Contents/jre/Contents/Home",
        )
    }

    let output = command
        .output()
        .map_err(|e| eyre!("failed to execute process: {}", e))?;

    let output = String::from_utf8(output.stdout)
        .map_err(|e| eyre!("failed to convert bytes to String: {}", e))?;

    let shemes = parse_schemes(&output);

    if shemes.is_empty() {
        return Ok(None);
    }

    Ok(Some(shemes))
}

fn parse_schemes(output: &str) -> Vec<String> {
    let mut variants = HashSet::new();
    for task in output.split("\n") {
        let assemble_task_pattern = Regex::new(r"(assemble)(\S+)").unwrap();
        let caps = assemble_task_pattern.captures(task);
        let Some(caps) = caps else {
            continue;
        };
        let Some(capture) = caps.get(2) else {
            continue;
        };
        let variant = capture.as_str().to_lowercase();
        if !variant.ends_with("test") {
            variants.insert(variant);
        }
    }

    let mut product_flavors = HashSet::new();
    for variant1 in variants.iter() {
        for variant2 in variants.iter() {
            if !(variant2.starts_with(variant1) && variant1 != variant2) {
                continue;
            }
            let build_type = variant2.clone().split_off(variant1.len());
            if variants.contains(&build_type) {
                product_flavors.insert(variant1.clone());
            }
        }
    }

    product_flavors
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
