use crate::model::{ErrorKind, JsonParseError, JsonValue};

/// Parses the JSON `null` literal with strict validation and error tracking.
///
/// # Arguments
///
/// * `input` - A string slice that may start with the `null` literal.
///
/// # Returns
///
/// * `Ok((JsonValue::Null, remaining_input))` if correctly parsed.
/// * `Err(JsonParseError)` if the token is malformed (e.g. `nulla`, `null123`).
/// * `Err(JsonParseError::unmatched(...))` if the input doesn't start with `null`.
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
                let pos = input.len() - rest.len();
                return Err(JsonParseError::new(
                    input,
                    pos,
                    ErrorKind::UnexpectedChar(c),
                ));
            }
        }
        Ok((JsonValue::Null, rest))
    } else {
        Err(JsonParseError::unmatched("null", input))
    }
}
