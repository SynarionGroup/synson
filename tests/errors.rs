use synson::parse_json;

#[test]
fn should_report_trailing_characters_error() {
    let err = parse_json("true false").unwrap_err();
    assert_eq!(err.message, "Trailing characters after JSON value");
    assert_eq!(err.index, 5);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 6);
}

#[test]
fn should_report_missing_colon_in_object() {
    let err = parse_json("{\"key\" \"value\"}").unwrap_err();
    assert_eq!(err.message, "Expected ':' after key in object");
    assert_eq!(err.line, 1);
}

#[test]
fn should_report_trailing_comma_in_array() {
    let err = parse_json("[true, false,]").unwrap_err();
    assert_eq!(err.message, "Trailing comma not allowed before ']'");
}

#[test]
fn should_report_unterminated_string() {
    let err = parse_json("\"unclosed string").unwrap_err();
    assert_eq!(err.message, "Unterminated string literal");
}

#[test]
fn should_report_bad_escape_sequence() {
    let err = parse_json("\"bad\\escape\"").unwrap_err();
    assert_eq!(err.message, "Invalid escape sequence in string");
}

#[test]
fn should_report_invalid_number_exponent() {
    let err = parse_json("1e").unwrap_err();
    assert_eq!(err.message, "Missing digits in exponent");
}

#[test]
fn should_report_leading_zero_error() {
    let err = parse_json("01").unwrap_err();
    assert_eq!(err.message, "Leading zeros are not allowed in numbers");
}
