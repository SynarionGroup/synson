use synson::{parse_bool, JsonValue};

#[test]
fn should_parse_true_and_false() {
    assert_eq!(parse_bool("true"), Ok((JsonValue::Bool(true), "")));
    assert_eq!(parse_bool("false"), Ok((JsonValue::Bool(false), "")));
    assert_eq!(
        parse_bool("false more"),
        Ok((JsonValue::Bool(false), " more"))
    );
}

#[test]
fn should_reject_invalid_booleans() {
    assert!(parse_bool("tru").is_err());
    assert!(parse_bool("truefalse").is_err());
    assert!(parse_bool("falsey").is_err());
}
