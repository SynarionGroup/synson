use crate::model::JsonValue;

/// Attempts to parse a JSON number (integers, floats and scientific notation).
///
/// # Arguments
///
/// * `input` - A string slice to parse.
///
/// # Returns
///
/// `Some((JsonValue::Number(value), remaining_input))` if successful, otherwise `None`.
pub fn parse_number(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    let mut chars = input.char_indices().peekable();
    let mut end_index = 0;
    let mut has_digits = false;

    // Optional minus sign
    if let Some((_, '-')) = chars.peek() {
        chars.next();
    }

    // Reject if leading with dot
    if let Some((_, '.')) = chars.peek() {
        return None;
    }

    // Integer part
    while let Some(&(i, c)) = chars.peek() {
        if c.is_ascii_digit() {
            has_digits = true;
            end_index = i + 1;
            chars.next();
        } else {
            break;
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
