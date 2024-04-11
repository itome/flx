use color_eyre::Result;
use json_comments::StripComments;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VSCodeLaunchConfigurationRequest {
    #[serde(rename = "launch")]
    Launch,
    #[serde(rename = "attach")]
    Attach,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VSCodeLaunchConfiguration {
    pub name: String,
    pub r#type: String,
    pub request: VSCodeLaunchConfigurationRequest,
    pub program: Option<String>,
    pub args: Option<Vec<String>>,
    pub cwd: Option<String>,
    #[serde(rename = "flutterMode")]
    pub flutter_mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VSCodeLaunchJson {
    pub version: String,
    pub configurations: Vec<VSCodeLaunchConfiguration>,
}

pub fn parse_launch_configuration(json: &str) -> Result<Vec<VSCodeLaunchConfiguration>> {
    // launch.json can have comments, so we need to strip them out
    let stripped = StripComments::new(json.as_bytes());
    let json: VSCodeLaunchJson = serde_json::from_reader(stripped)?;
    Ok(json.configurations)
}

#[cfg(test)]
mod test {
    use crate::launch::VSCodeLaunchConfiguration;

    #[test]
    fn parse_launch_1() {
        let contents = include_str!("./test/launch_1.json");
        let result = super::parse_launch_configuration(contents).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            VSCodeLaunchConfiguration {
                name: "Launch Development".to_string(),
                r#type: "dart".to_string(),
                request: super::VSCodeLaunchConfigurationRequest::Launch,
                program: Some("lib/main/main_development.dart".to_string()),
                args: Some(vec![
                    "--flavor".to_string(),
                    "development".to_string(),
                    "--target".to_string(),
                    "lib/main/main_development.dart".to_string(),
                    "--dart-define".to_string(),
                ]),
                cwd: None,
                flutter_mode: None,
            }
        );
        assert_eq!(
            result[1],
            VSCodeLaunchConfiguration {
                name: "Launch Production".to_string(),
                r#type: "dart".to_string(),
                request: super::VSCodeLaunchConfigurationRequest::Launch,
                program: Some("lib/main/main_production.dart".to_string()),
                args: Some(vec![
                    "--flavor".to_string(),
                    "production".to_string(),
                    "--target".to_string(),
                    "lib/main/main_production.dart".to_string(),
                ]),
                cwd: None,
                flutter_mode: Some("release".to_string()),
            }
        );
        assert_eq!(
            result[2],
            VSCodeLaunchConfiguration {
                name: "Launch UI Gallery".to_string(),
                r#type: "dart".to_string(),
                request: super::VSCodeLaunchConfigurationRequest::Launch,
                program: Some("packages/app_ui/gallery/lib/main.dart".to_string()),
                args: None,
                cwd: None,
                flutter_mode: None,
            }
        )
    }
}
