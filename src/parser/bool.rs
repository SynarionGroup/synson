use crate::model::{JsonParseError, JsonValue};

/// Parses a JSON boolean literal (`true` or `false`) with strict syntax validation.
///
/// Ensures the value is fully formed and not followed by alphanumeric characters
/// (e.g. `trueX` or `falsehood` are rejected).
///
/// # Arguments
///
/// * `input` - A string slice expected to start with a JSON boolean.
///
/// # Returns
///
/// * `Ok((JsonValue::Bool(value), remaining_input))` if successfully parsed.
/// * `Err(JsonParseError)` if the token is malformed (e.g. `falsey`, `true123`).
/// * `Err(JsonParseError::unmatched("boolean", input))` if the input is unrelated.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_bool;
/// use synson::model::JsonValue;
///
/// assert_eq!(parse_bool("true "), Ok((JsonValue::Bool(true), " ")));
/// assert_eq!(parse_bool("false"), Ok((JsonValue::Bool(false), "")));
/// assert!(parse_bool("tru").is_err());
/// assert!(parse_bool("falsehood").is_err());
/// ```
pub fn parse_bool(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let input = input.trim_start();

    if let Some(rest) = input.strip_prefix("true") {
        if let Some(c) = rest.chars().next() {
            if c.is_ascii_alphanumeric() {
                return Err(JsonParseError::new(
                    "Invalid token after 'true'",
                    input.len() - rest.len(),
                    input,
                ));
            }
        }
        return Ok((JsonValue::Bool(true), rest));
    }

    if let Some(rest) = input.strip_prefix("false") {
        if let Some(c) = rest.chars().next() {
            if c.is_ascii_alphanumeric() {
                return Err(JsonParseError::new(
                    "Invalid token after 'false'",
                    input.len() - rest.len(),
                    input,
                ));
            }
        }
        return Ok((JsonValue::Bool(false), rest));
    }

    Err(JsonParseError::unmatched("boolean", input))
}
