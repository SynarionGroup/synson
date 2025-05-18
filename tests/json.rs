use std::collections::HashMap;

use synson::model::{ErrorKind, JsonParseOptions};
use synson::{parse_json, JsonValue};

#[test]
fn should_parse_valid_json_values() {
    assert_eq!(parse_json("null", None), Ok(JsonValue::Null));
    assert_eq!(parse_json("true ", None), Ok(JsonValue::Bool(true)));
    assert_eq!(
        parse_json("\"ok\"", None),
        Ok(JsonValue::String("ok".to_string()))
    );
    assert_eq!(parse_json("42", None), Ok(JsonValue::Number(42.0)));
    assert_eq!(
        parse_json("[1, false]", None),
        Ok(JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Bool(false)
        ]))
    );
}

#[test]
fn should_reject_invalid_json_values() {
    assert!(parse_json("nul", None).is_err());
    assert!(parse_json("truefalse", None).is_err());
    assert!(parse_json("42 garbage", None).is_err());
    assert!(parse_json("}", None).is_err());
}

#[test]
fn should_report_trailing_characters_error() {
    let err = parse_json("true false", None).unwrap_err();
    assert!(matches!(
        err.kind,
        ErrorKind::Custom(ref msg) if msg.contains("Trailing characters")
    ));
    assert_eq!(err.index, 5);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 6);
}

#[test]
fn should_report_trailing_characters_after_object() {
    let err = parse_json("{\"key\": true} extra", None).unwrap_err();
    assert!(matches!(
        err.kind,
        ErrorKind::Custom(ref msg) if msg.contains("Trailing characters")
    ));
    assert_eq!(err.index, 14);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 15);
}

#[test]
fn should_report_trailing_characters_error_in_strict_mode() {
    let err = parse_json("true false", Some(&JsonParseOptions::strict())).unwrap_err();
    assert!(matches!(
        err.kind,
        ErrorKind::Custom(ref msg) if msg.contains("Trailing characters")
    ));
    assert_eq!(err.index, 5);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 6);
}

#[test]
fn should_report_trailing_characters_after_object_in_strict_mode() {
    let err = parse_json("{\"key\": true} extra", Some(&JsonParseOptions::strict())).unwrap_err();
    assert!(matches!(
        err.kind,
        ErrorKind::Custom(ref msg) if msg.contains("Trailing characters")
    ));
    assert_eq!(err.index, 14);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 15);
}

#[test]
fn should_not_report_trailing_characters_in_tolerant_mode() {
    let result = parse_json("true false", Some(&JsonParseOptions::tolerant()));
    assert!(result.is_ok());
}

#[test]
fn should_serialize_null() {
    assert_eq!(JsonValue::Null.to_json(), "null");
}

#[test]
fn should_serialize_bool() {
    assert_eq!(JsonValue::Bool(true).to_json(), "true");
    assert_eq!(JsonValue::Bool(false).to_json(), "false");
}

#[test]
fn should_serialize_number() {
    assert_eq!(JsonValue::Number(42.0).to_json(), "42");
    assert_eq!(JsonValue::Number(-42.42).to_json(), "-42.42");
}

#[test]
fn should_serialize_string_with_escaping() {
    let s = JsonValue::String("Hello\n\"World\"\\".to_string());
    assert_eq!(s.to_json(), "\"Hello\\n\\\"World\\\"\\\\\"");
}

#[test]
fn should_serialize_array() {
    let arr = JsonValue::Array(vec![
        JsonValue::Bool(true),
        JsonValue::Number(1.0),
        JsonValue::String("ok".to_string()),
    ]);
    assert_eq!(arr.to_json(), "[true,1,\"ok\"]");
}

#[test]
fn should_serialize_object() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), JsonValue::Number(1.0));
    map.insert("b".to_string(), JsonValue::Bool(false));
    let obj = JsonValue::Object(map);
    assert_eq!(obj.to_json(), "{\"a\":1,\"b\":false}");
}
