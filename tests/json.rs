use synson::{parse_json, JsonValue};

#[test]
fn should_parse_valid_json_values() {
    assert_eq!(parse_json("null"), Ok(JsonValue::Null));
    assert_eq!(parse_json("true "), Ok(JsonValue::Bool(true)));
    assert_eq!(
        parse_json("\"ok\""),
        Ok(JsonValue::String("ok".to_string()))
    );
    assert_eq!(parse_json("42"), Ok(JsonValue::Number(42.0)));
    assert_eq!(
        parse_json("[1, false]"),
        Ok(JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Bool(false)
        ]))
    );
}

#[test]
fn should_reject_invalid_json_values() {
    assert!(parse_json("nul").is_err());
    assert!(parse_json("truefalse").is_err());
    assert!(parse_json("42 garbage").is_err());
    assert!(parse_json("}").is_err());
}
