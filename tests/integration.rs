use std::collections::HashMap;
use synson::{parse_json, JsonValue};

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
