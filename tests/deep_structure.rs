use std::fs;
use synson::model::JsonParseOptions;
use synson::parse_json;

fn load_json_from_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read JSON file")
}

#[test]
fn should_parse_valid_deep_structure() {
    let json_data = load_json_from_file("tests/assets/valid_deep_structure.json");
    let result = parse_json(&json_data, Some(&JsonParseOptions::default()));
    assert!(result.is_ok());
}

#[test]
fn should_fail_on_invalid_deep_structure() {
    let json_data = load_json_from_file("tests/assets/invalid_deep_structure.json");
    let result = parse_json(&json_data, Some(&JsonParseOptions::default()));
    assert!(result.is_err());
}

#[test]
fn should_parse_large_json_structure() {
    let json_data = load_json_from_file("tests/assets/large_deep_structure.json");
    let result = parse_json(&json_data, Some(&JsonParseOptions::default()));
    assert!(result.is_ok());
}

#[test]
fn should_handle_invalid_json_syntax() {
    let json_data = load_json_from_file("tests/assets/invalid_syntax.json");
    let result = parse_json(&json_data, Some(&JsonParseOptions::default()));
    assert!(result.is_err());
}
