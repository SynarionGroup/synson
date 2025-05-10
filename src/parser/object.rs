use crate::model::{JsonParseError, JsonValue};
use crate::parser::parse_string;

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
    let mut input = input.trim_start(); // mutable, pas "let input = ..."

    println!("parse_object input: '{}'", input);

    if !input.starts_with('{') {
        return Err(JsonParseError::new(
            "Expected '{' to start object",
            0,
            input,
        ));
    }

    input = &input[1..]; // skip '{'
    let mut map = HashMap::new();

    loop {
        input = input.trim_start();
        println!("parse_object loop input: '{}'", input);

        if let Some(rest) = input.strip_prefix('}') {
            return Ok((JsonValue::Object(map), rest));
        }

        let (key_value, rest) = parse_string(input)
            .map_err(|e| JsonParseError::new("Expected string key in object", e.index, input))?;

        let JsonValue::String(key) = key_value else {
            return Err(JsonParseError::new("Object keys must be strings", 0, input));
        };
        println!("parse_object key: {:?}", key);

        input = rest.trim_start();

        if !input.starts_with(':') {
            let offset = original_input.len() - input.len();
            return Err(JsonParseError::new(
                "Expected ':' after key in object",
                offset,
                original_input,
            ));
        }

        input = &input[1..]; // skip ':'
        input = input.trim_start();

        let (value, rest) = parse_value(input)?;
        map.insert(key, value);
        input = rest.trim_start();

        if let Some(rest) = input.strip_prefix(',') {
            input = rest;

            if input.trim_start().starts_with('}') {
                return Err(JsonParseError::new(
                    "Trailing comma not allowed before '}'",
                    input.len() - input.trim_start().len(),
                    input,
                ));
            }

            continue;
        } else if let Some(rest) = input.strip_prefix('}') {
            return Ok((JsonValue::Object(map), rest));
        } else {
            return Err(JsonParseError::new(
                "Expected ',' or '}' after object entry",
                input.len(),
                input,
            ));
        }
    }
}
