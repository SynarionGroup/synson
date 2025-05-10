use synson::{parse_number, JsonValue};

#[test]
fn should_parse_valid_numbers() {
    assert_eq!(parse_number("0"), Ok((JsonValue::Number(0.0), "")));
    assert_eq!(parse_number("42 "), Ok((JsonValue::Number(42.0), " ")));
    assert_eq!(parse_number("-12.5,"), Ok((JsonValue::Number(-12.5), ",")));

    // Exponentials
    assert_eq!(parse_number("1e3"), Ok((JsonValue::Number(1000.0), "")));
    assert_eq!(parse_number("2E2 "), Ok((JsonValue::Number(200.0), " ")));
    assert_eq!(parse_number("-3.5e-1"), Ok((JsonValue::Number(-0.35), "")));
    assert_eq!(parse_number("0.5e+2"), Ok((JsonValue::Number(50.0), "")));

    // Edge cases
    assert_eq!(parse_number("-0"), Ok((JsonValue::Number(-0.0), "")));
    assert_eq!(parse_number("0.0"), Ok((JsonValue::Number(0.0), "")));
}

#[test]
fn should_reject_invalid_numbers() {
    // Empty or incomplete
    assert!(parse_number("").is_err());
    assert!(parse_number("-").is_err());
    assert!(parse_number(".").is_err());

    // Invalid sequences
    assert!(parse_number("12abc").is_err());
    assert!(parse_number("--1").is_err());
    assert!(parse_number("1.2.3").is_err());

    // Exponentials without digits
    assert!(parse_number("1e").is_err());
    assert!(parse_number("1e+").is_err());
    assert!(parse_number("1e-").is_err());
    assert!(parse_number("1e+2.3").is_err());
    assert!(parse_number("1e-2.3").is_err());

    // Invalid suffix
    assert!(parse_number("3.1415rest").is_err());

    // Leading dot or trailing dot
    assert!(parse_number(".5").is_err());
    assert!(parse_number("5.").is_err());

    // Leading zero violations
    assert!(parse_number("01").is_err());
    assert!(parse_number("00").is_err());
}
