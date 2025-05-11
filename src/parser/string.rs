use crate::model::{JsonParseError, JsonValue};

/// Parses a JSON string literal with escape support (`\\`, `\"`, `\n`, `\t`, `\/`, `\b`, `\f`, `\r`).
///
/// Does **not** support `\uXXXX` (reserved for a future version).
///
/// # Arguments
///
/// * `input` - A string slice expected to start with a double quote (`"`).
///
/// # Returns
///
/// * `Ok((JsonValue::String(value), remaining_input))` if a valid JSON string is parsed.
/// * `Err(JsonParseError)` if the string is malformed or escape sequences are invalid.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_string;
/// use synson::model::JsonValue;
///
/// assert_eq!(
///     parse_string("\"line\\nbreak\""),
///     Ok((JsonValue::String("line\nbreak".to_string()), ""))
/// );
/// assert_eq!(
///     parse_string("\"slash\\/escape\""),
///     Ok((JsonValue::String("slash/escape".to_string()), ""))
/// );
/// assert!(parse_string("\"unterminated").is_err());
/// assert!(parse_string("\"bad\\escape\"").is_err());
/// ```
pub fn parse_string(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let input = input.trim_start();
    let mut chars = input.char_indices();

    let (_, first) = chars
        .next()
        .ok_or_else(|| JsonParseError::new("Expected '\"' to start string", 0, input))?;

    if first != '"' {
        return Err(JsonParseError::new(
            "Expected '\"' to start string",
            0,
            input,
        ));
    }

    let mut result = String::new();
    let mut escape = false;

    for (i, c) in chars {
        if escape {
            match c {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                '/' => result.push('/'),
                'b' => result.push('\u{0008}'),
                'f' => result.push('\u{000C}'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                _ => {
                    return Err(JsonParseError::new(
                        "Invalid escape sequence in string",
                        i,
                        input,
                    ));
                }
            }
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == '"' {
            let rest = &input[i + 1..];
            return Ok((JsonValue::String(result), rest));
        } else {
            result.push(c);
        }
    }

    Err(JsonParseError::new(
        "Unterminated string literal",
        input.len(),
        input,
    ))
}
