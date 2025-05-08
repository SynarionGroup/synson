use synson::{parse_bool, JsonValue};

#[test]
fn should_parse_true_and_false() {
    assert_eq!(parse_bool("true"), Some((JsonValue::Bool(true), "")));
    assert_eq!(parse_bool("false"), Some((JsonValue::Bool(false), "")));
    assert_eq!(
        parse_bool("false more"),
        Some((JsonValue::Bool(false), " more"))
    );
}

#[test]
fn should_reject_invalid_booleans() {
    assert_eq!(parse_bool("tru"), None);
    assert_eq!(parse_bool("truefalse"), None);
    assert_eq!(parse_bool("falsey"), None);
}
