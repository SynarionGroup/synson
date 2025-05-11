use synson::model::JsonParseOptions;
use synson::{parse_json, JsonValue};

#[test]
fn should_handle_fuzz_inputs() {
    // Test valid JSON with strange spaces
    assert_eq!(
        parse_json("   true    ", Some(&JsonParseOptions::default())),
        Ok(JsonValue::Bool(true))
    );
    assert_eq!(
        parse_json("  \"test string\"  ", Some(&JsonParseOptions::default())),
        Ok(JsonValue::String("test string".to_string()))
    );

    // Test empty string
    assert_eq!(
        parse_json("\"\" ", Some(&JsonParseOptions::default())),
        Ok(JsonValue::String("".to_string()))
    );

    // Test large numbers (scientific notation) - Use f64::INFINITY for testing extreme values
    assert_eq!(
        parse_json("1e308", Some(&JsonParseOptions::default())),
        Ok(JsonValue::Number(1e308))
    );
    assert_eq!(
        parse_json("-1e308", Some(&JsonParseOptions::default())),
        Ok(JsonValue::Number(-1e308))
    );

    // Test invalid JSON that should be rejected
    assert!(parse_json("   invalid_input", Some(&JsonParseOptions::default())).is_err());
    assert!(parse_json("1.0.0", Some(&JsonParseOptions::default())).is_err());
    assert!(parse_json("[1, 2,, 3]", Some(&JsonParseOptions::default())).is_err());

    // Ensure that even with strange formatting, the parser works correctly
    assert_eq!(
        parse_json(
            "    { \"key\" : true }   ",
            Some(&JsonParseOptions::default())
        ),
        Ok(JsonValue::Object({
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), JsonValue::Bool(true));
            map
        }))
    );
}
