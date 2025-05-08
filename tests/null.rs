use synson::{parse_null, JsonValue};

#[test]
fn should_parse_null_correctly() {
    assert_eq!(parse_null("null"), Some((JsonValue::Null, "")));
    assert_eq!(parse_null("null "), Some((JsonValue::Null, " ")));
    assert_eq!(parse_null("null\tmore"), Some((JsonValue::Null, "\tmore")));
}

#[test]
fn should_fail_on_invalid_null() {
    assert_eq!(parse_null("nul"), None);
    assert_eq!(parse_null("nulla"), None);
}
