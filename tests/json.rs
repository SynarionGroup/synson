use synson::model::JsonParseOptions;
use synson::{parse_json, JsonValue};

#[test]
fn should_parse_valid_json_values() {
    assert_eq!(
        parse_json("null", None),
        Ok(JsonValue::Null)
    );
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
    assert_eq!(err.message, "Trailing characters after JSON value");
    assert_eq!(err.index, 5);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 6);
}

#[test]
fn should_report_trailing_characters_after_object() {
    let err = parse_json("{\"key\": true} extra", None).unwrap_err();
    assert_eq!(err.message, "Trailing characters after JSON value");
    assert_eq!(err.index, 14);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 15);
}

#[test]
fn should_report_trailing_characters_error_in_strict_mode() {
    let err = parse_json("true false", Some(&JsonParseOptions::strict())).unwrap_err();
    assert_eq!(err.message, "Trailing characters after JSON value");
    assert_eq!(err.index, 5);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 6);
}

#[test]
fn should_report_trailing_characters_after_object_in_strict_mode() {
    let err = parse_json("{\"key\": true} extra", Some(&JsonParseOptions::strict())).unwrap_err();
    assert_eq!(err.message, "Trailing characters after JSON value");
    assert_eq!(err.index, 14);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 15);
}

#[test]
fn should_not_report_trailing_characters_in_tolerant_mode() {
    let result = parse_json("true false", Some(&JsonParseOptions::tolerant()));
    assert!(result.is_ok());
}
