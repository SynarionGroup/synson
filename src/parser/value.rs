use super::{parse_array, parse_bool, parse_null, parse_number, parse_object, parse_string};
use crate::model::{ErrorKind, JsonParseError, JsonValue};

/// Parses any valid JSON value, including primitives and nested arrays/objects.
///
/// This is the core dispatch function used by composite parsers. It attempts each
/// value type in order and returns the first successful parse. If no value matches,
/// a detailed `JsonParseError` is returned.
///
/// # Arguments
///
/// * `input` - A string slice starting at the next JSON token.
///
/// # Returns
///
/// * `Ok((JsonValue, remaining_input))` if a valid value is found.
/// * `Err(JsonParseError)` if none match or an inner parser fails.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_value;
/// use synson::model::JsonValue;
///
/// assert_eq!(parse_value("true"), Ok((JsonValue::Bool(true), "")));
/// assert!(parse_value("!invalid").is_err());
/// ```
pub fn parse_value(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let mut first_expected_err: Option<JsonParseError> = None;

    for parser in [
        parse_object,
        parse_array,
        parse_string,
        parse_number,
        parse_bool,
        parse_null,
    ] {
        match parser(input) {
            Ok(ok) => return Ok(ok),
            Err(e) => match &e.kind {
                ErrorKind::Custom(msg) if msg.starts_with("Expected") => {
                    if first_expected_err.is_none() {
                        first_expected_err = Some(e);
                    }
                    continue;
                }
                ErrorKind::UnexpectedChar(_) => {
                    if first_expected_err.is_none() {
                        first_expected_err = Some(e);
                    }
                    continue;
                }
                _ => return Err(e),
            },
        }
    }

    Err(first_expected_err.unwrap_or_else(|| JsonParseError::unmatched("value", input)))
}
