use crate::model::{JsonParseError, JsonValue};

/// Parses the JSON `null` literal with strict validation and error tracking.
///
/// # Arguments
///
/// * `input` - A string slice that may start with the `null` literal.
///
/// # Returns
///
/// * `Ok((JsonValue::Null, remaining_input))` if correctly parsed.
/// * `Err(JsonParseError)` if it looks like a `null` but is malformed.
/// * `Err(JsonParseError::unmatched(...))` if it’s not `null` at all.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_null;
/// use synson::model::JsonValue;
///
/// assert_eq!(parse_null("null "), Ok((JsonValue::Null, " ")));
/// assert!(parse_null("nul").is_err());
/// assert!(parse_null("nulla").is_err());
/// ```
pub fn parse_null(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let input = input.trim_start();

    if let Some(rest) = input.strip_prefix("null") {
        if let Some(c) = rest.chars().next() {
            if c.is_ascii_alphanumeric() {
                return Err(JsonParseError::new(
                    "Unexpected character after 'null'",
                    input.len() - rest.len(),
                    input,
                ));
            }
        }
        Ok((JsonValue::Null, rest))
    } else {
        Err(JsonParseError::unmatched("null", input))
    }
}
