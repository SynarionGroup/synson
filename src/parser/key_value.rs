use super::{parse_string, parse_value};
use crate::model::{ErrorKind, JsonParseError, JsonValue};

/// Parses a single `"key": value` pair from a JSON object context.
///
/// This is a low-level utility used for streaming or partial parsing use-cases.
///
/// # Arguments
///
/// * `input` - A string slice expected to start with a quoted string key.
///
/// # Returns
///
/// * `Ok(((key, value), remaining_input))` on success
/// * `Err(JsonParseError)` if syntax is invalid or malformed.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_key_value_pair;
/// use synson::model::JsonValue;
///
/// let input = r#""name": "John""#;
/// let ((key, value), rest) = parse_key_value_pair(input).unwrap();
/// assert_eq!(key, "name");
/// assert_eq!(value, JsonValue::String("John".into()));
/// assert_eq!(rest.trim_start(), "");
/// ```
pub fn parse_key_value_pair(input: &str) -> Result<((String, JsonValue), &str), JsonParseError> {
    let original = input;
    let input = input.trim_start();

    let (key_val, rest) = parse_string(input)
        .map_err(|e| JsonParseError::new(input, e.index, ErrorKind::ExpectedStringKey))?;

    let JsonValue::String(key) = key_val else {
        return Err(JsonParseError::new(
            original,
            0,
            ErrorKind::InvalidObjectKey,
        ));
    };

    let rest = rest.trim_start();

    if !rest.starts_with(':') {
        let offset = original.len() - rest.len();
        return Err(JsonParseError::new(
            original,
            offset,
            ErrorKind::ExpectedColon,
        ));
    }

    let rest = &rest[1..];
    let rest = rest.trim_start();

    let (value, remaining) = parse_value(rest)?;
    Ok(((key, value), remaining))
}
