use synson::{parse_string, JsonValue};

#[test]
fn should_parse_basic_strings() {
    assert_eq!(
        parse_string("\"hello\""),
        Ok((JsonValue::String("hello".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"with space\" "),
        Ok((JsonValue::String("with space".to_string()), " "))
    );
}

#[test]
fn should_parse_escaped_strings() {
    assert_eq!(
        parse_string("\"line\\nbreak\""),
        Ok((JsonValue::String("line\nbreak".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"tab\\tseparated\""),
        Ok((JsonValue::String("tab\tseparated".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"quote: \\\"\""),
        Ok((JsonValue::String("quote: \"".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"backslash: \\\\\""),
        Ok((JsonValue::String("backslash: \\".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"back\\bspace\""),
        Ok((JsonValue::String("back\u{0008}space".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"form\\ffeed\""),
        Ok((JsonValue::String("form\u{000C}feed".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"carriage\\rreturn\""),
        Ok((JsonValue::String("carriage\rreturn".to_string()), ""))
    );
    assert_eq!(
        parse_string("\"escaped\\/slash\""),
        Ok((JsonValue::String("escaped/slash".to_string()), ""))
    );
}

#[test]
fn should_reject_invalid_strings() {
    assert!(parse_string("not a string").is_err());
    assert!(parse_string("\"unclosed").is_err());
    assert!(parse_string("\"bad\\escape\"").is_err());
}

#[test]
fn should_parse_unicode_escaped_strings() {
    assert_eq!(
        parse_string("\"unicode smile: \\u263A\""),
        Ok((JsonValue::String("unicode smile: ☺".to_string()), "")),
    );

    assert_eq!(
        parse_string("\"greek letter: \\u03A9\""),
        Ok((JsonValue::String("greek letter: Ω".to_string()), "")),
    );
}

#[test]
fn should_parse_unicode_surrogate_pairs() {
    assert_eq!(
        parse_string("\"emoji: \\uD83D\\uDE00\""),
        Ok((JsonValue::String("emoji: 😀".to_string()), "")),
    );

    assert_eq!(
        parse_string("\"another emoji: \\uD83D\\uDE80\""),
        Ok((JsonValue::String("another emoji: 🚀".to_string()), "")),
    );
}

#[test]
fn should_reject_invalid_unicode_escapes() {
    assert!(parse_string("\"bad unicode: \\uZZZZ\"").is_err());
    assert!(parse_string("\"orphan surrogate: \\uD83D\"").is_err());
    assert!(parse_string("\"lonely low surrogate: \\uDE00\"").is_err());
}
