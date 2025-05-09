use crate::model::JsonValue;

/// Parses a JSON number, including integers, decimals, and scientific notation (e.g. `1e3`, `-2.5E-2`).
///
/// The parser enforces strict compliance with JSON number syntax:
/// - No leading zeros (e.g. `01` is invalid)
/// - Fractional parts must have at least one digit (e.g. `1.` is invalid)
/// - Exponential parts must be complete (e.g. `1e`, `1e+` are invalid)
///
/// # Arguments
///
/// * `input` - A string slice expected to start with a valid number.
///
/// # Returns
///
/// `Some((JsonValue::Number(value), remaining_input))` if the number is valid, otherwise `None`.
///
/// # Examples
///
/// ```
/// use synson::{parse_number, JsonValue};
///
/// assert_eq!(parse_number("42"), Some((JsonValue::Number(42.0), "")));
/// assert_eq!(parse_number("-3.14"), Some((JsonValue::Number(-3.14), "")));
/// assert_eq!(parse_number("1e2"), Some((JsonValue::Number(100.0), "")));
/// assert_eq!(parse_number("01"), None); // invalid leading zero
/// assert_eq!(parse_number("2e"), None); // incomplete exponent
/// ```
pub fn parse_number(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    let mut chars = input.char_indices().peekable();
    let mut end_index = 0;
    let mut has_digits = false;

    // Optional minus sign
    if let Some((_, '-')) = chars.peek() {
        chars.next();
    }

    // Reject leading dot
    if let Some((_, '.')) = chars.peek() {
        return None;
    }

    // Integer part
    let mut leading_zero = false;
    if let Some(&(i, c)) = chars.peek() {
        if c == '0' {
            leading_zero = true;
            has_digits = true;
            end_index = i + 1;
            chars.next();
        } else if c.is_ascii_digit() {
            while let Some(&(i, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    has_digits = true;
                    end_index = i + 1;
                    chars.next();
                } else {
                    break;
                }
            }
        } else {
            return None;
        }
    }

    // Reject leading zeros like "01"
    if leading_zero {
        if let Some(&(_, c)) = chars.peek() {
            if c.is_ascii_digit() {
                return None;
            }
        }
    }

    // Fractional part
    if let Some(&(_, '.')) = chars.peek() {
        chars.next(); // consume '.'

        let mut has_frac_digits = false;
        while let Some(&(j, c)) = chars.peek() {
            if c.is_ascii_digit() {
                has_digits = true;
                has_frac_digits = true;
                end_index = j + 1;
                chars.next();
            } else {
                break;
            }
        }

        if !has_frac_digits {
            return None;
        }
    }

    if !has_digits {
        return None;
    }

    // Exponent part
    if let Some(&(_, 'e' | 'E')) = chars.peek() {
        chars.next(); // consume 'e' or 'E'

        // Optional sign
        if let Some(&(_, '+' | '-')) = chars.peek() {
            chars.next();
        }

        let mut has_exp_digits = false;
        while let Some(&(i, c)) = chars.peek() {
            if c.is_ascii_digit() {
                has_exp_digits = true;
                end_index = i + 1;
                chars.next();
            } else {
                break;
            }
        }

        if !has_exp_digits {
            return None;
        }
    }

    let (matched, rest) = input.split_at(end_index);

    // Trailing invalid char
    if let Some(c) = rest.chars().next() {
        if c.is_ascii_alphabetic() || c == '.' {
            return None;
        }
    }

    matched
        .parse::<f64>()
        .ok()
        .map(|num| (JsonValue::Number(num), rest))
}
