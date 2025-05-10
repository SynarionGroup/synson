use crate::model::{JsonParseError, JsonValue};
use crate::parser::parse_value;

/// Parses a complete JSON value from a string slice, ensuring full input consumption.
///
/// This is the main entry point for parsing JSON in Synson. It accepts any valid JSON type
/// (`null`, `true`, `false`, numbers, strings, arrays, objects`) and returns either the parsed value
/// or a detailed `JsonParseError` with line, column, and index.
///
/// The parser enforces strict compliance with JSON syntax:
/// - The entire input must be consumed (no trailing characters)
/// - Invalid constructs (e.g. malformed numbers or unterminated strings) are rejected
///
/// # Arguments
///
/// * `input` - A UTF-8 string slice representing the full JSON input.
///
/// # Returns
///
/// * `Ok(JsonValue)` if parsing succeeds and all input is consumed.
/// * `Err(JsonParseError)` if parsing fails or extra content remains.
///
/// # Examples
///
/// ```
/// use synson::{parse_json, JsonValue};
///
/// assert_eq!(parse_json("42"), Ok(JsonValue::Number(42.0)));
/// assert_eq!(parse_json(" true "), Ok(JsonValue::Bool(true)));
///
/// assert!(parse_json("invalid").is_err());
/// assert!(parse_json("123 trailing").is_err());
/// ```
pub fn parse_json(input: &str) -> Result<JsonValue, JsonParseError> {
    let trimmed_input = input.trim_start();
    println!("parse_json input: '{}'", input); // Log de l'input initial

    let (value, rest) = parse_value(trimmed_input)?;

    let rest_trimmed = rest.trim_start();
    println!("parse_json value: {:?}", value); // Log de la valeur analysée
    println!("parse_json rest: '{}'", rest); // Log de la partie restante

    if !rest_trimmed.is_empty() {
        let offset = trimmed_input.len() - rest_trimmed.len();
        println!(
            "parse_json error: Trailing characters found at offset {}",
            offset
        );
        return Err(JsonParseError::new(
            "Trailing characters after JSON value",
            offset,
            input,
        ));
    }

    Ok(value)
}
