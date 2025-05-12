use std::collections::HashMap;
use synson::{parse_json, JsonParseOptions, JsonValue};

fn build_expected() -> JsonValue {
    let mut map = HashMap::new();
    map.insert(
        "key".to_string(),
        JsonValue::Array(vec![
            JsonValue::Null,
            JsonValue::String("ok".to_string()),
            JsonValue::Bool(false),
        ]),
    );
    JsonValue::Object(map)
}

#[test]
fn should_parse_full_valid_jsons() {
    assert_eq!(
        parse_json("{\"key\": [null, \"ok\", false]}", None),
        Ok(build_expected())
    );

    assert_eq!(
        parse_json(" { \"key\" : [ null , \"ok\" , false ] } ", None),
        Ok(build_expected())
    );

    assert_eq!(
        parse_json("{\"a\": []}", None),
        Ok(JsonValue::Object({
            let mut map = HashMap::new();
            map.insert("a".to_string(), JsonValue::Array(vec![]));
            map
        }))
    );

    assert_eq!(
        parse_json("{\"a\": {}}", None),
        Ok(JsonValue::Object({
            let mut map = HashMap::new();
            map.insert("a".to_string(), JsonValue::Object(HashMap::new()));
            map
        }))
    );
}

#[test]
fn should_fail_on_invalid_full_jsons() {
    assert!(parse_json("{\"key\" \"missing colon\"}", None).is_err());
    assert!(parse_json("[1, 2,]", None).is_err());
    assert!(parse_json("{\"key\": [true,]", None).is_err());
    assert!(parse_json("{\"key\": 123", None).is_err());
    assert!(parse_json("{\"a\": 1 \"b\": 2}", None).is_err());
    assert!(parse_json("{\"a\": true, \"b\"}", None).is_err());
    assert!(parse_json("{\"a\" 1}", None).is_err());
    assert!(parse_json("{\"a\": [1, null", None).is_err());
}

#[test]
fn should_preserve_value_after_roundtrip() {
    let json = "{\"msg\":\"Hello\",\"val\":[true,42,null]}";
    let parsed = parse_json(json, Some(&JsonParseOptions::default())).expect("valid json");
    let serialized = parsed.to_json();
    let reparsed =
        parse_json(&serialized, Some(&JsonParseOptions::default())).expect("valid json again");
    assert_eq!(parsed, reparsed);
}
