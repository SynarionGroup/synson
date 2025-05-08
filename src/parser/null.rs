use crate::JsonValue;

/// Attempts to parse the JSON `null` literal.
///
/// # Arguments
///
/// * `input` - A string slice that should start with "null".
///
/// # Returns
///
/// `Some((JsonValue::Null, remaining_input))` if successful, otherwise `None`.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_null;
/// use synson::JsonValue;
///
/// assert_eq!(
///     parse_null("null "),
///     Some((JsonValue::Null, " "))
/// );
/// assert_eq!(parse_null("nul"), None);
/// ```
pub fn parse_null(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    if let Some(rest) = input.strip_prefix("null") {
        match rest.chars().next() {
            Some(c) if c.is_ascii_alphanumeric() => None,
            _ => Some((JsonValue::Null, rest)),
        }
    } else {
        None
    }
}
