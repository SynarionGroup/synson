use crate::model::{ErrorKind, JsonParseError, JsonValue};

/// Parses a JSON number, including integers, decimals, and scientific notation (e.g. `1e3`, `-2.5E-2`).
///
/// The parser enforces strict JSON number formatting rules:
/// - No leading zeros for integers (e.g. `01` is invalid)
/// - A fractional part must have at least one digit (e.g. `5.` is invalid)
/// - An exponent must have digits (e.g. `1e`, `1e+` are invalid)
///
/// # Arguments
///
/// * `input` - A string slice expected to start with a valid JSON number.
///
/// # Returns
///
/// * `Ok((JsonValue::Number(value), remaining_input))` if a valid number is parsed.
/// * `Err(JsonParseError)` if the number is invalid or malformed.
/// * `Err(JsonParseError::unmatched(...))` if the input is unrelated to a number.
///
/// # Examples
///
/// ```
/// use synson::parser::parse_number;
/// use synson::model::JsonValue;
///
/// assert_eq!(parse_number("42"), Ok((JsonValue::Number(42.0), "")));
/// assert_eq!(parse_number("-3.14"), Ok((JsonValue::Number(-3.14), "")));
/// assert_eq!(parse_number("1e2"), Ok((JsonValue::Number(100.0), "")));
/// assert!(parse_number("01").is_err());
/// assert!(parse_number("abc").is_err());
/// ```
pub fn parse_number(input: &str) -> Result<(JsonValue, &str), JsonParseError> {
    let input = input.trim_start();
    let mut chars = input.char_indices().peekable();
    let original = input;
    let mut end_index = 0;

    // Optional minus sign
    if let Some((_, '-')) = chars.peek() {
        chars.next();
    }

    // Reject leading dot (e.g. `.5` is invalid in JSON)
    if let Some((_, '.')) = chars.peek() {
        return Err(JsonParseError::unmatched("number", input));
    }

    // Integer part
    let mut leading_zero = false;
    if let Some(&(i, c)) = chars.peek() {
        if c == '0' {
            leading_zero = true;
            end_index = i + 1;
            chars.next();
        } else if c.is_ascii_digit() {
            while let Some(&(i, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    end_index = i + 1;
                    chars.next();
                } else {
                    break;
                }
            }
        } else {
            return Err(JsonParseError::unmatched("number", input));
        }
    } else {
        return Err(JsonParseError::unmatched("number", input));
    }

    // Reject leading zeros like "01"
    if leading_zero {
        if let Some(&(_, c)) = chars.peek() {
            if c.is_ascii_digit() {
                return Err(JsonParseError::new(
                    original,
                    end_index,
                    ErrorKind::LeadingZero,
                ));
            }
        }
    }

    // Fractional part
    if let Some(&(_, '.')) = chars.peek() {
        chars.next(); // consume '.'
        let mut has_frac_digits = false;
        while let Some(&(j, c)) = chars.peek() {
            if c.is_ascii_digit() {
                has_frac_digits = true;
                end_index = j + 1;
                chars.next();
            } else {
                break;
            }
        }
        if !has_frac_digits {
            return Err(JsonParseError::new(
                original,
                end_index,
                ErrorKind::DecimalWithoutDigits,
            ));
        }
    }

    // Exponent part
    if let Some(&(_, 'e' | 'E')) = chars.peek() {
        chars.next(); // consume 'e' or 'E'
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
            return Err(JsonParseError::new(
                original,
                end_index,
                ErrorKind::InvalidExponent,
            ));
        }
    }

    let (matched, rest) = input.split_at(end_index);

    if let Some(c) = rest.chars().next() {
        if c.is_ascii_alphabetic() || c == '.' {
            return Err(JsonParseError::new(
                original,
                end_index,
                ErrorKind::UnexpectedChar(c),
            ));
        }
    }

    matched
        .parse::<f64>()
        .map(|num| (JsonValue::Number(num), rest))
        .map_err(|_| JsonParseError::new(original, end_index, ErrorKind::InvalidNumber))
}
