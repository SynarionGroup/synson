use crate::model::{JsonParseError, JsonValue};
use crate::parser::{
    parse_array, parse_bool, parse_null, parse_number, parse_object, parse_string,
};

/// Entry point for parsing a JSON value from input.
///
/// # Arguments
///
/// * `input` - A full JSON string.
///
/// # Returns
///
/// `Ok(JsonValue)` if successfully parsed and all input consumed,
/// otherwise `Err(JsonParseError)`.
///
/// # Examples
///
/// ```
/// use synson::{parse_json, JsonValue};
///
/// assert_eq!(
///     parse_json(" 42 "),
///     Ok(JsonValue::Number(42.0))
/// );
///
/// assert_eq!(
///     parse_json("true"),
///     Ok(JsonValue::Bool(true))
/// );
/// ```
pub fn parse_json(input: &str) -> Result<JsonValue, JsonParseError> {
    let input = input.trim_start();

    let (value, rest) = parse_null(input)
        .or_else(|| parse_bool(input))
        .or_else(|| parse_number(input))
        .or_else(|| parse_string(input))
        .or_else(|| parse_array(input))
        .or_else(|| parse_object(input))
        .ok_or_else(|| JsonParseError::new("Unrecognized JSON value", 0))?;

    if !rest.trim_start().is_empty() {
        let offset = input.len() - rest.trim_start().len();
        return Err(JsonParseError::new(
            "Trailing characters after JSON value",
            offset,
        ));
    }

    Ok(value)
}
