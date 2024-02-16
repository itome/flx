use std::fs;

#[test]
fn get_xcode_project_info() {
    let contents = include_str!("./test/xcodebuild.txt");
    let info = super::XcodeProjectInfo::new_from_xcode_build_output(&contents);
    assert!(info.targets.contains(&"Runner"));
    assert!(!info.targets.contains(&"Undefined"));

    assert!(info.build_configurations.contains(&"Config1"));
    assert!(info.build_configurations.contains(&"Config2"));
    assert!(info.build_configurations.contains(&"Config3"));
    assert!(!info.build_configurations.contains(&"Undefined"));

    assert!(!info.targets.contains(&"Scheme1"));
    assert!(!info.targets.contains(&"Scheme2"));
    assert!(!info.targets.contains(&"Scheme3"));
    assert!(!info.targets.contains(&"Undefined"));
}
