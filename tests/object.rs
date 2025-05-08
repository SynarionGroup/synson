use std::collections::HashMap;
use synson::{parse_object, JsonValue};

#[test]
fn should_parse_simple_objects() {
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), JsonValue::Number(1.0));
    expected.insert("b".to_string(), JsonValue::Bool(true));
    expected.insert("c".to_string(), JsonValue::String("ok".to_string()));

    assert_eq!(
        parse_object("{\"a\": 1, \"b\": true, \"c\": \"ok\"}"),
        Some((JsonValue::Object(expected), ""))
    );

    assert_eq!(
        parse_object("{ }"),
        Some((JsonValue::Object(HashMap::new()), ""))
    );
}

#[test]
fn should_reject_invalid_objects() {
    assert_eq!(parse_object("{a: 1}"), None);
    assert_eq!(parse_object("{\"a\" 1}"), None);
    assert_eq!(parse_object("{\"a\": 1,}"), None);
    assert_eq!(parse_object("{\"a\": 1 \"b\": 2}"), None);
    assert_eq!(parse_object("not an object"), None);
}
