use synson::{parse_key_value_pair, JsonValue};

#[test]
fn should_parse_valid_key_value_pairs() {
    assert_eq!(
        parse_key_value_pair(r#""key": 42"#),
        Ok((("key".to_string(), JsonValue::Number(42.0)), ""))
    );

    assert_eq!(
        parse_key_value_pair(r#""active": true"#),
        Ok((("active".to_string(), JsonValue::Bool(true)), ""))
    );

    assert_eq!(
        parse_key_value_pair(r#""name": "synson""#),
        Ok((
            ("name".to_string(), JsonValue::String("synson".to_string())),
            ""
        ))
    );

    assert_eq!(
        parse_key_value_pair(r#""data": [1, 2, 3]"#),
        Ok((
            (
                "data".to_string(),
                JsonValue::Array(vec![
                    JsonValue::Number(1.0),
                    JsonValue::Number(2.0),
                    JsonValue::Number(3.0)
                ])
            ),
            ""
        ))
    );
}

#[test]
fn should_reject_invalid_keys_or_syntax() {
    assert!(parse_key_value_pair(r#"123: "bad""#).is_err());
    assert!(parse_key_value_pair(r#""key" "missing_colon""#).is_err());
    assert!(parse_key_value_pair(r#""key": "#).is_err());
    assert!(parse_key_value_pair(r#""key": invalid"#).is_err());
}
