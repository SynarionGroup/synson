use crate::model::{ErrorKind, JsonParseError, JsonParseOptions, JsonValue};
use crate::parser::parse_value;

/// Parses a complete JSON value from a string slice, ensuring full input consumption.
///
/// This function is the main entry point for parsing JSON data in Synson. It accepts any valid JSON type,
/// including `null`, `true`, `false`, numbers, strings, arrays, and objects, and returns either the parsed value
/// or a detailed `JsonParseError` indicating the failure.
///
/// The parser enforces strict compliance with JSON syntax:
/// - The entire input must be consumed (no trailing characters)
/// - Invalid constructs (e.g., malformed numbers or unterminated strings) are rejected
///
/// The behavior of the parser can be controlled using the `JsonParseOptions` structure.
/// If no options are provided, the parser defaults to strict mode.
///
/// # Arguments
///
/// * `input` - A UTF-8 string slice representing the full JSON input. It should be a valid JSON string.
/// * `options` - An optional reference to a `JsonParseOptions` struct, which defines whether the parser should
///   operate in strict or tolerant mode. If `None`, the default is strict mode.
///
/// # Returns
///
/// * `Ok(JsonValue)` if the parsing succeeds and all input is consumed.
/// * `Err(JsonParseError)` if parsing fails or extra content remains after the parsing is completed.
///
/// # Examples
///
/// ```
/// use synson::{parse_json, JsonValue};
/// use synson::model::JsonParseOptions;
///
/// // Strict mode, no trailing characters allowed
/// let result = parse_json("true false", None);  // Default to strict
/// assert!(result.is_err());
///
/// // Tolerant mode, trailing characters are allowed
/// let result = parse_json("true false", Some(&JsonParseOptions::tolerant()));
/// assert!(result.is_ok());
/// ```
pub fn parse_json(
    input: &str,
    options: Option<&JsonParseOptions>,
) -> Result<JsonValue, JsonParseError> {
    let trimmed_input = input.trim_start();
    let (value, rest) = parse_value(trimmed_input)?;

    let rest_trimmed = rest.trim_start();
    let default_options = JsonParseOptions::default();
    let options = options.unwrap_or(&default_options);

    if !rest_trimmed.is_empty() && options.strict {
        let offset = input.len() - rest_trimmed.len();
        return Err(JsonParseError::new(
            input,
            offset,
            ErrorKind::Custom("Trailing characters after JSON value".into()),
        ));
    }

    Ok(value)
}
