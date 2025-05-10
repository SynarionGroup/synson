use super::parse_value;
use crate::model::{JsonParseError, JsonValue};

/// Parses a JSON array with potentially nested values and precise error tracking.
///
/// # Arguments
///
/// * `input` - A string slice expected to start with `'['`.
///
/// # Returns
///
/// `Ok((JsonValue::Array(vec), remaining_input))` if successful,
/// otherwise `Err(JsonParseError)` with position info.
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

        if let Some(rest) = remaining.strip_prefix(']') {
            return Ok((JsonValue::Array(values), rest));
        }

        // Parse value
        let (value, rest) =
            parse_value(remaining).map_err(|e| JsonParseError::new(&e.message, e.index, input))?;
        values.push(value);
        remaining = rest.trim_start();

        if let Some(rest) = remaining.strip_prefix(',') {
            remaining = rest;

            if remaining.trim_start().starts_with(']') {
                let err_pos = input.len() - remaining.trim_start().len();
                return Err(JsonParseError::new(
                    "Trailing comma not allowed before ']'",
                    err_pos,
                    input,
                ));
            }

            continue;
        } else if let Some(rest) = remaining.strip_prefix(']') {
            return Ok((JsonValue::Array(values), rest));
        } else {
            let err_pos = input.len() - remaining.len();
            return Err(JsonParseError::new(
                "Expected ',' or ']' after array element",
                err_pos,
                input,
            ));
        }
    }
}
