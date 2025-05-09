use crate::model::JsonValue;

/// Parses a JSON string literal with full escape sequence support (except `\uXXXX`).
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
///     parse_string("\"line\\nbreak\""),
///     Some((JsonValue::String("line\nbreak".to_string()), ""))
/// );
/// assert_eq!(
///     parse_string("\"\\b\\f\\r\""),
///     Some((JsonValue::String("\u{0008}\u{000C}\r".to_string()), ""))
/// );
/// assert_eq!(
///     parse_string("\"slash\\/escape\""),
///     Some((JsonValue::String("slash/escape".to_string()), ""))
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
                '/' => result.push('/'),
                'b' => result.push('\u{0008}'),
                'f' => result.push('\u{000C}'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
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
