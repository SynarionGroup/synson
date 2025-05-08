use super::{parse_array, parse_bool, parse_null, parse_number, parse_object, parse_string};
use crate::model::JsonValue;

/// Attempts to parse any valid JSON value.
pub fn parse_value(input: &str) -> Option<(JsonValue, &str)> {
    parse_null(input)
        .or_else(|| parse_bool(input))
        .or_else(|| parse_number(input))
        .or_else(|| parse_string(input))
        .or_else(|| parse_array(input))
        .or_else(|| parse_object(input))
}
