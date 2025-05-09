use crate::model::JsonValue;

use super::parse_value;

/// Attempts to parse a JSON array with potentially nested values.
///
/// # Arguments
///
/// * `input` - A string slice expected to start with `'['`.
///
/// # Returns
///
/// `Some((JsonValue::Array(vec), remaining_input))` if successful, otherwise `None`.
///
/// # Examples
///
/// ```
/// use synson::{parse_array, JsonValue};
///
/// assert_eq!(
///     parse_array("[1, {\"a\": [true, false]}, 3]"),
///     Some((
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
pub fn parse_array(input: &str) -> Option<(JsonValue, &str)> {
    let mut input = input.trim_start();

    if !input.starts_with('[') {
        return None;
    }
    input = &input[1..];

    let mut values = Vec::new();

    loop {
        input = input.trim_start();

        if let Some(rest) = input.strip_prefix(']') {
            return Some((JsonValue::Array(values), rest));
        }

        let (value, rest) = parse_value(input)?;
        values.push(value);
        input = rest.trim_start();

        if let Some(rest) = input.strip_prefix(',') {
            input = rest;

            if input.trim_start().starts_with(']') {
                return None;
            }

            continue;
        } else if let Some(rest) = input.strip_prefix(']') {
            return Some((JsonValue::Array(values), rest));
        } else {
            return None;
        }
    }
}
