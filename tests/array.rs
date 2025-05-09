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
    assert_eq!(parse_array("[1, true,]"), None);
    assert_eq!(parse_array("[1 true]"), None);
    assert_eq!(parse_array("[1, "), None);
    assert_eq!(parse_array("not an array"), None);
}

#[test]
fn should_parse_nested_arrays_and_objects() {
    use std::collections::HashMap;

    let result = parse_array(r#"[1, {"a": [true, false]}, 3]"#);
    assert_eq!(
        result,
        Some((
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Object({
                    let mut m = HashMap::new();
                    m.insert(
                        "a".to_string(),
                        JsonValue::Array(vec![JsonValue::Bool(true), JsonValue::Bool(false)]),
                    );
                    m
                }),
                JsonValue::Number(3.0),
            ]),
            ""
        ))
    );

    let result = parse_array(r#"[{"x": 1}, {"x": {"y": [2, 3]}}]"#);
    assert_eq!(
        result,
        Some((
            JsonValue::Array(vec![
                JsonValue::Object({
                    let mut m = HashMap::new();
                    m.insert("x".to_string(), JsonValue::Number(1.0));
                    m
                }),
                JsonValue::Object({
                    let mut inner = HashMap::new();
                    inner.insert(
                        "y".to_string(),
                        JsonValue::Array(vec![JsonValue::Number(2.0), JsonValue::Number(3.0)]),
                    );
                    let mut m = HashMap::new();
                    m.insert("x".to_string(), JsonValue::Object(inner));
                    m
                }),
            ]),
            ""
        ))
    );
}
