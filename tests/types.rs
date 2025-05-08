use synson::JsonValue;

#[test]
fn should_construct_simple_json_value() {
    let value = JsonValue::String("hello".to_string());
    assert_eq!(value, JsonValue::String(String::from("hello")));
}
