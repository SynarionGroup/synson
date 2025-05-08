use crate::model::JsonValue;

/// Attempts to parse a JSON boolean literal (`true` or `false`).
///
/// # Arguments
///
/// * `input` - A string slice to parse.
///
/// # Returns
///
/// `Some((JsonValue::Bool(value), remaining_input))` if successful, otherwise `None`.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_bool;
/// use synson::JsonValue;
///
/// assert_eq!(parse_bool("true "), Some((JsonValue::Bool(true), " ")));
/// assert_eq!(parse_bool("false"), Some((JsonValue::Bool(false), "")));
/// assert_eq!(parse_bool("fals"), None);
/// ```
pub fn parse_bool(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();

    if let Some(rest) = input.strip_prefix("true") {
        if let Some(c) = rest.chars().next() {
            if c.is_ascii_alphanumeric() {
                return None;
            }
        }
        return Some((JsonValue::Bool(true), rest));
    }

    if let Some(rest) = input.strip_prefix("false") {
        if let Some(c) = rest.chars().next() {
            if c.is_ascii_alphanumeric() {
                return None;
            }
        }
        return Some((JsonValue::Bool(false), rest));
    }

    None
}
