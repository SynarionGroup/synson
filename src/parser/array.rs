use super::parse_value;
use crate::model::{ErrorKind, JsonParseError, JsonValue};

/// Parses a JSON array with potentially nested values and precise error tracking.
///
/// # Arguments
///
/// * `input` - A string slice expected to start with `'['`.
///
/// # Returns
///
/// * `Ok((JsonValue::Array(values), remaining_input))` if successfully parsed.
/// * `Err(JsonParseError)` if the input is malformed (e.g. missing commas, bad values).
///
/// # Examples
///
/// ```
/// use synson::{parse_array, JsonValue};
///
/// assert_eq!(
///     parse_array("[1, {\"a\": [true, false]}, 3]"),
///     Ok((
///         JsonValue::Array(vec![
///             JsonValue::Number(1.0),
///             JsonValue::Object({
///                 let mut map = std::collections::HashMap::new();
///                 map.insert("a".to_string(), JsonValue::Array(vec![JsonValue::Bool(true), JsonValue::Bool(false)]));
///                 map
///             }),
///             JsonValue::Number(3.0)
///         ]),
///         ""
///     ))
/// );
/// ```
pub fn parse_array(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let input = input.trim_start();

    if !input.starts_with('[') {
        return Err(JsonParseError::unmatched("array", input));
    }

    let mut remaining = &input[1..]; // skip '['
    let mut values = Vec::new();

    loop {
        remaining = remaining.trim_start();

        // Empty array or finished
        if let Some(rest) = remaining.strip_prefix(']') {
            return Ok((JsonValue::Array(values), rest));
        }

        // Try to parse a value
        let (value, rest) = parse_value(remaining)
            .map_err(|e| JsonParseError::new(input, e.index, ErrorKind::ExpectedValue))?;

        values.push(value);
        remaining = rest.trim_start();

        if let Some(rest) = remaining.strip_prefix(',') {
            remaining = rest;

            // Trailing comma is forbidden
            if remaining.trim_start().starts_with(']') {
                let err_pos = input.len() - remaining.trim_start().len();
                return Err(JsonParseError::new(
                    input,
                    err_pos,
                    ErrorKind::TrailingComma,
                ));
            }

            continue;
        } else if let Some(rest) = remaining.strip_prefix(']') {
            return Ok((JsonValue::Array(values), rest));
        } else {
            let err_pos = input.len() - remaining.len();
            return Err(JsonParseError::new(
                input,
                err_pos,
                ErrorKind::ExpectedComma,
            ));
        }
    }
}
