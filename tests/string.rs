use synson::{parse_string, JsonValue};

#[test]
fn should_parse_basic_strings() {
    assert_eq!(
        parse_string("\"hello\""),
        Some((JsonValue::String("hello".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"with space\" "),
        Some((JsonValue::String("with space".to_string()), " "))
    );
}

#[test]
fn should_parse_escaped_strings() {
    assert_eq!(
        parse_string("\"line\\nbreak\""),
        Some((JsonValue::String("line\nbreak".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"tab\\tseparated\""),
        Some((JsonValue::String("tab\tseparated".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"quote: \\\"\""),
        Some((JsonValue::String("quote: \"".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"backslash: \\\\\""),
        Some((JsonValue::String("backslash: \\".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"back\\bspace\""),
        Some((JsonValue::String("back\u{0008}space".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"form\\ffeed\""),
        Some((JsonValue::String("form\u{000C}feed".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"carriage\\rreturn\""),
        Some((JsonValue::String("carriage\rreturn".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"escaped\\/slash\""),
        Some((JsonValue::String("escaped/slash".to_string()), ""))
    );
}

#[test]
fn should_reject_invalid_strings() {
    assert_eq!(parse_string("not a string"), None);
    assert_eq!(parse_string("\"unclosed"), None);
    assert_eq!(parse_string("\"bad\\escape\""), None);
}
