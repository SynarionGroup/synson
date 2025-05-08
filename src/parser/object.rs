use crate::model::JsonValue;
use crate::parser::parse_string;

use std::collections::HashMap;

use super::parse_value;

/// Attempts to parse a JSON object with string keys and simple values.
///
/// # Arguments
///
/// * `input` - A string slice expected to start with `'{'`.
///
/// # Returns
///
/// `Some((JsonValue::Object(map), remaining_input))` if successful, otherwise `None`.
///
/// # Examples
///
/// ```
/// use synson::{parse_object, JsonValue};
/// use std::collections::HashMap;
///
/// let mut expected = HashMap::new();
/// expected.insert("a".to_string(), JsonValue::Number(1.0));
///
/// assert_eq!(
///     parse_object("{\"a\": 1}"),
///     Some((JsonValue::Object(expected), ""))
/// );
/// ```
pub fn parse_object(input: &str) -> Option<(JsonValue, &str)> {
    let mut input = input.trim_start();

    if !input.starts_with('{') {
        return None;
    }
    input = &input[1..];

    let mut map = HashMap::new();

    loop {
        input = input.trim_start();

        if let Some(rest) = input.strip_prefix('}') {
            return Some((JsonValue::Object(map), rest));
        }

        // Parse key
        let (JsonValue::String(key), rest) = parse_string(input)? else {
            return None;
        };

        input = rest.trim_start();

        // Expect colon
        input = input.strip_prefix(':')?.trim_start();

        // Parse value
        let (value, rest) = parse_value(input)?;
        map.insert(key, value);
        input = rest.trim_start();

        if let Some(rest) = input.strip_prefix(',') {
            input = rest;

            if input.trim_start().starts_with('}') {
                return None;
            }

            continue;
        } else if let Some(rest) = input.strip_prefix('}') {
            return Some((JsonValue::Object(map), rest));
        } else {
            return None;
        }
    }
}
