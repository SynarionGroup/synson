use synson::{parse_array, JsonValue};

#[test]
fn should_parse_simple_arrays() {
    assert_eq!(
        parse_array("[1, true, \"ok\"]"),
        Some((
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Bool(true),
                JsonValue::String("ok".to_string())
            ]),
            ""
        ))
    );

    assert_eq!(parse_array(" [ ] "), Some((JsonValue::Array(vec![]), " ")));
}

#[test]
fn should_reject_malformed_arrays() {
    assert_eq!(parse_array("[1, true,]"), None); // trailing comma
    assert_eq!(parse_array("[1 true]"), None); // missing comma
    assert_eq!(parse_array("[1, "), None); // unterminated
    assert_eq!(parse_array("not an array"), None);
}
