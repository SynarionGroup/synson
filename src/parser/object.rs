use crate::model::{ErrorKind, JsonParseError, JsonValue};

use std::collections::HashMap;

use super::parse_value;

/// Parses a JSON object with string keys and potentially nested values.
///
/// This parser enforces strict syntax rules:
/// - Keys must be quoted strings
/// - A colon must follow each key
/// - Commas separate key-value pairs
/// - Trailing commas are rejected
///
/// # Arguments
///
/// * `input` - A string slice expected to start with `'{'`.
///
/// # Returns
///
/// * `Ok((JsonValue::Object(map), remaining_input))` if valid.
/// * `Err(JsonParseError)` if the syntax is invalid at any point.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_object;
/// use synson::model::JsonValue;
/// use std::collections::HashMap;
///
/// let mut expected = HashMap::new();
/// expected.insert("user".to_string(), JsonValue::Object({
///     let mut inner = HashMap::new();
///     inner.insert("id".to_string(), JsonValue::Number(1.0));
///     inner.insert("tags".to_string(), JsonValue::Array(vec![
///         JsonValue::String("rust".to_string()),
///         JsonValue::String("json".to_string()),
///     ]));
///     inner
/// }));
///
/// assert_eq!(
///     parse_object("{\"user\": {\"id\": 1, \"tags\": [\"rust\", \"json\"]}}"),
///     Ok((JsonValue::Object(expected), ""))
/// );
/// ```
pub fn parse_object(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let original_input = input;
    let mut input = input.trim_start();

    if !input.starts_with('{') {
        return Err(JsonParseError::new(
            input,
            0,
            ErrorKind::UnexpectedChar(input.chars().next().unwrap_or('?')),
        ));
    }

    input = &input[1..];
    let mut map = HashMap::new();

    loop {
        input = input.trim_start();

        if let Some(rest) = input.strip_prefix('}') {
            return Ok((JsonValue::Object(map), rest));
        }

        // Try full JSON value (to catch non-string keys like numbers)
        let (key_val, rest) = parse_value(input)?;

        match key_val {
            JsonValue::String(key) => {
                input = rest.trim_start();

                if !input.starts_with(':') {
                    let offset = original_input.len() - input.len();
                    return Err(JsonParseError::new(
                        original_input,
                        offset,
                        ErrorKind::ExpectedColon,
                    ));
                }

                input = &input[1..];
                input = input.trim_start();

                let (value, rest) = parse_value(input)?;
                map.insert(key, value);
                input = rest.trim_start();

                if let Some(rest) = input.strip_prefix(',') {
                    input = rest;

                    if input.trim_start().starts_with('}') {
                        let pos = original_input.len() - input.trim_start().len();
                        return Err(JsonParseError::new(
                            original_input,
                            pos,
                            ErrorKind::TrailingComma,
                        ));
                    }

                    continue;
                } else if let Some(rest) = input.strip_prefix('}') {
                    return Ok((JsonValue::Object(map), rest));
                } else {
                    let pos = original_input.len() - input.len();
                    return Err(JsonParseError::new(
                        original_input,
                        pos,
                        ErrorKind::ExpectedComma,
                    ));
                }
            }
            _ => {
                let offset = original_input.len() - input.len();
                return Err(JsonParseError::new(
                    original_input,
                    offset,
                    ErrorKind::InvalidObjectKey,
                ));
            }
        }
    }
}
