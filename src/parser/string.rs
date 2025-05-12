use crate::model::{JsonParseError, JsonValue};

/// Parses a JSON string literal with escape support (`\\`, `\"`, `\n`, `\t`, `\/`, `\b`, `\f`, `\r`, `\uXXXX`).
///
/// Supports Unicode escapes (`\u263A`) and surrogate pairs (`\uD83D\uDE00`).
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
    let mut i = 1;

    while i < input.len() {
        let c = input[i..].chars().next().unwrap();
        let char_len = c.len_utf8();

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
                'u' => {
                    if i + 5 >= input.len() {
                        return Err(JsonParseError::new(
                            "Incomplete Unicode escape sequence",
                            i,
                            input,
                        ));
                    }
                    let hex = &input[i + 1..i + 5];
                    let codepoint = u16::from_str_radix(hex, 16)
                        .map_err(|_| JsonParseError::new("Invalid Unicode escape", i, input))?;

                    i += 4;

                    if (0xD800..=0xDBFF).contains(&codepoint) {
                        if i + 2 >= input.len() || &input[i + 1..i + 3] != "\\u" {
                            return Err(JsonParseError::new(
                                "Expected low surrogate after high surrogate",
                                i,
                                input,
                            ));
                        }

                        let low_hex = &input[i + 3..i + 7];
                        let low_codepoint = u16::from_str_radix(low_hex, 16)
                            .map_err(|_| JsonParseError::new("Invalid low surrogate", i, input))?;

                        if !(0xDC00..=0xDFFF).contains(&low_codepoint) {
                            return Err(JsonParseError::new(
                                "Invalid low surrogate value",
                                i,
                                input,
                            ));
                        }

                        let high_ten = (codepoint - 0xD800) as u32;
                        let low_ten = (low_codepoint - 0xDC00) as u32;
                        let scalar = 0x10000 + ((high_ten << 10) | low_ten);

                        let ch = char::from_u32(scalar).ok_or_else(|| {
                            JsonParseError::new("Invalid Unicode scalar value", i, input)
                        })?;

                        result.push(ch);
                        i += 6;
                    } else {
                        let ch = char::from_u32(codepoint as u32).ok_or_else(|| {
                            JsonParseError::new("Invalid Unicode codepoint", i, input)
                        })?;
                        result.push(ch);
                    }
                }
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
            let rest = &input[i + char_len..];
            return Ok((JsonValue::String(result), rest));
        } else {
            result.push(c);
        }

        i += char_len;
    }

    Err(JsonParseError::new(
        "Unterminated string literal",
        input.len(),
        input,
    ))
}
