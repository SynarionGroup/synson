use synson::{parse_number, JsonValue};

#[test]
fn should_parse_valid_numbers() {
    assert_eq!(parse_number("0"), Some((JsonValue::Number(0.0), "")));
    assert_eq!(parse_number("42 "), Some((JsonValue::Number(42.0), " ")));
    assert_eq!(
        parse_number("-12.5,"),
        Some((JsonValue::Number(-12.5), ","))
    );

    // Exponentials
    assert_eq!(parse_number("1e3"), Some((JsonValue::Number(1000.0), "")));
    assert_eq!(parse_number("2E2 "), Some((JsonValue::Number(200.0), " ")));
    assert_eq!(
        parse_number("-3.5e-1"),
        Some((JsonValue::Number(-0.35), ""))
    );
    assert_eq!(parse_number("0.5e+2"), Some((JsonValue::Number(50.0), "")));

    // Edge cases
    assert_eq!(parse_number("-0"), Some((JsonValue::Number(-0.0), "")));
    assert_eq!(parse_number("0.0"), Some((JsonValue::Number(0.0), "")));
}

#[test]
fn should_reject_invalid_numbers() {
    // Empty or incomplete
    assert_eq!(parse_number(""), None);
    assert_eq!(parse_number("-"), None);
    assert_eq!(parse_number("."), None);

    // Invalid sequences
    assert_eq!(parse_number("12abc"), None);
    assert_eq!(parse_number("--1"), None);
    assert_eq!(parse_number("1.2.3"), None);

    // Exponentials without digits
    assert_eq!(parse_number("1e"), None);
    assert_eq!(parse_number("1e+"), None);
    assert_eq!(parse_number("1e-"), None);
    assert_eq!(parse_number("1e+2.3"), None);
    assert_eq!(parse_number("1e-2.3"), None);

    // Invalid suffix
    assert_eq!(parse_number("3.1415rest"), None);

    // Leading dot or trailing dot
    assert_eq!(parse_number(".5"), None);
    assert_eq!(parse_number("5."), None);

    // Leading zero violations
    assert_eq!(parse_number("01"), None);
    assert_eq!(parse_number("00"), None);
}
