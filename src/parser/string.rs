use crate::model::JsonValue;

/// Attempts to parse a JSON string literal with basic escapes.
///
/// # Arguments
///
/// * `input` - A string slice starting with a double quote (`"`).
///
/// # Returns
///
/// `Some((JsonValue::String(value), remaining_input))` if successful, otherwise `None`.
///
/// # Examples
///
/// ```
/// use synson::{parse_string, JsonValue};
///
/// assert_eq!(
///     parse_string("\"hello\""),
///     Some((JsonValue::String("hello".to_string()), ""))
/// );
///
/// assert_eq!(
///     parse_string("\"line\\nbreak\""),
///     Some((JsonValue::String("line\nbreak".to_string()), ""))
/// );
/// ```
pub fn parse_string(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    let mut chars = input.char_indices();

    let (_, first) = chars.next()?;
    if first != '"' {
        return None;
    }

    let mut result = String::new();
    let mut escape = false;

    for (i, c) in chars {
        if escape {
            match c {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                'n' => result.push('\n'),
                't' => result.push('\t'),
                _ => return None,
            }
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == '"' {
            let rest = &input[i + 1..];
            return Some((JsonValue::String(result), rest));
        } else {
            result.push(c);
        }
    }

    None
}
