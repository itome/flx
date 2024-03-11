use std::fs;

#[test]
pub fn parse_schemes() {
    let contents = include_str!("./test/gradlew.txt");
    let schemes = super::parse_schemes(contents);
    assert!(schemes.contains(&"develop".to_string()));
    assert!(schemes.contains(&"staging".to_string()));
    assert!(schemes.contains(&"production".to_string()));
    assert!(!schemes.contains(&"undefined".to_string()));
}
