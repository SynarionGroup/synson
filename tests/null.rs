use synson::{parse_null, JsonValue};

#[test]
fn should_parse_null_correctly() {
    assert_eq!(parse_null("null"), Ok((JsonValue::Null, "")));
    assert_eq!(parse_null("null "), Ok((JsonValue::Null, " ")));
    assert_eq!(parse_null("null\tmore"), Ok((JsonValue::Null, "\tmore")));
}

#[test]
fn should_fail_on_invalid_null() {
    assert!(parse_null("nul").is_err());
    assert!(parse_null("nulla").is_err());
}
