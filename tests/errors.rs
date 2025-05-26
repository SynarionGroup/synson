use synson::model::ErrorKind;
use synson::parse_json;

#[test]
fn should_report_trailing_characters_error() {
    let err = parse_json("true false", None).unwrap_err();
    assert!(matches!(
        err.kind,
        ErrorKind::Custom(ref msg) if msg.contains("Trailing characters")
    ));
    assert_eq!(err.index, 5);
    assert_eq!(err.line, 1);
    assert_eq!(err.column, 6);
}

#[test]
fn should_report_missing_colon_in_object() {
    let err = parse_json("{\"key\" \"value\"}", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::ExpectedColon));
    assert_eq!(err.line, 1);
}

#[test]
fn should_report_trailing_comma_in_array() {
    let err = parse_json("[true, false,]", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::TrailingComma));
}

#[test]
fn should_report_unterminated_string() {
    let err = parse_json("\"unclosed string", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::UnterminatedString));
}

#[test]
fn should_report_bad_escape_sequence() {
    let err = parse_json("\"bad\\escape\"", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::InvalidEscape));
}

#[test]
fn should_report_invalid_number_exponent() {
    let err = parse_json("1e", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::InvalidExponent));
}

#[test]
fn should_report_leading_zero_error() {
    let err = parse_json("01", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::LeadingZero));
}

#[test]
fn should_reject_non_string_object_keys() {
    let err = parse_json("{123: true}", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::InvalidObjectKey));
}

#[test]
fn should_reject_double_comma_in_array() {
    let err = parse_json("[1,,2]", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::ExpectedValue));
}

#[test]
fn should_reject_completely_invalid_start() {
    let err = parse_json("!invalid", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::UnexpectedChar('!')));
}

#[test]
fn should_report_invalid_unicode_escape() {
    let err = parse_json("\"\\uXYZ1\"", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::InvalidUnicode));
}

#[test]
fn should_report_invalid_surrogate_pair() {
    let err = parse_json("\"\\uD83D\\u1234\"", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::InvalidUnicode));
}

#[test]
fn should_reject_lonely_high_surrogate() {
    let err = parse_json("\"\\uD800\"", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::InvalidUnicode));
}

#[test]
fn should_reject_decimal_with_no_digits() {
    let err = parse_json("1.", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::DecimalWithoutDigits));
}

#[test]
fn should_reject_duplicate_object_keys() {
    let err = parse_json("{\"a\": 1, \"a\": 2}", None).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::DuplicateKey(ref key) if key == "a"));
}
