use crate::model::JsonValue;
use crate::parser::{parse_bool, parse_null, parse_number, parse_string};

/// Attempts to parse a JSON array of simple values (null, bool, number, string).
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
///     parse_array("[1, true, \"ok\"]"),
///     Some((
///         JsonValue::Array(vec![
///             JsonValue::Number(1.0),
///             JsonValue::Bool(true),
///             JsonValue::String("ok".to_string())
///         ]),
///         ""
///     ))
/// );
///
/// assert_eq!(
///     parse_array("[ ]"),
///     Some((JsonValue::Array(vec![]), ""))
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

fn parse_value(input: &str) -> Option<(JsonValue, &str)> {
    parse_null(input)
        .or_else(|| parse_bool(input))
        .or_else(|| parse_number(input))
        .or_else(|| parse_string(input))
}
