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
        Ok((JsonValue::Object(expected), ""))
    );

    assert_eq!(
        parse_object("{ }"),
        Ok((JsonValue::Object(HashMap::new()), ""))
    );
}

#[test]
fn should_reject_invalid_objects() {
    assert!(parse_object("{a: 1}").is_err());
    assert!(parse_object("{\"a\" 1}").is_err());
    assert!(parse_object("{\"a\": 1,}").is_err());
    assert!(parse_object("{\"a\": 1 \"b\": 2}").is_err());
    assert!(parse_object("not an object").is_err());
}

#[test]
fn should_parse_nested_objects_and_arrays() {
    let mut inner = HashMap::new();
    inner.insert("id".to_string(), JsonValue::Number(1.0));
    inner.insert(
        "tags".to_string(),
        JsonValue::Array(vec![
            JsonValue::String("rust".to_string()),
            JsonValue::String("json".to_string()),
        ]),
    );

    let mut expected = HashMap::new();
    expected.insert("user".to_string(), JsonValue::Object(inner));

    assert_eq!(
        parse_object(r#"{"user": {"id": 1, "tags": ["rust", "json"]}}"#),
        Ok((JsonValue::Object(expected), ""))
    );
}

#[test]
fn should_parse_deeply_nested_objects() {
    let mut d = HashMap::new();
    d.insert("d".to_string(), JsonValue::Null);

    let mut c = HashMap::new();
    c.insert("c".to_string(), JsonValue::Object(d));

    let mut b = HashMap::new();
    b.insert("b".to_string(), JsonValue::Object(c));

    let mut a = HashMap::new();
    a.insert("a".to_string(), JsonValue::Object(b));

    assert_eq!(
        parse_object(r#"{"a": {"b": {"c": {"d": null}}}}"#),
        Ok((JsonValue::Object(a), ""))
    );
}
