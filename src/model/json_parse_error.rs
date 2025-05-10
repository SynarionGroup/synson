/// Represents an error encountered while parsing JSON.
#[derive(Debug, PartialEq)]
pub struct JsonParseError {
    pub message: String,
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl JsonParseError {
    /// Creates a new `JsonParseError` with computed line and column information.
    ///
    /// This constructor calculates the human-readable location (line and column)
    /// corresponding to a byte index in the original JSON input. Useful for surfacing
    /// precise error locations in CLI tools, editors, or debugging output.
    ///
    /// # Arguments
    ///
    /// * `message` - A description of the parsing error.
    /// * `position` - The byte index in the input string where the error occurred.
    /// * `input` - The original JSON input string.
    ///
    /// # Returns
    ///
    /// A fully populated `JsonParseError` with message, byte index, line, and column.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::json_parse_error::JsonParseError;
    ///
    /// let err = JsonParseError::new("Unexpected token", 15, "{ \"a\": tru }");
    /// assert_eq!(err.message, "Unexpected token");
    /// assert_eq!(err.index, 15);
    /// assert_eq!(err.line, 1);
    /// assert_eq!(err.column, 13);
    /// ```
    pub fn new(message: &str, position: usize, input: &str) -> Self {
        let mut line = 1;
        let mut column = 1;
        for (_, c) in input.char_indices().take(position) {
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Self {
            message: message.to_string(),
            index: position,
            line,
            column,
        }
    }

    /// Creates a generic fallback error when a value parser fails to match its expected literal.
    ///
    /// This should only be used when the parser is *not* confident that the input was intended
    /// for this type (e.g. `parse_null` called on `{"key": 1}`).
    ///
    /// # Arguments
    ///
    /// * `expected` - The name of the expected literal (e.g. `"null"`).
    /// * `input` - The input slice being parsed.
    ///
    /// # Returns
    ///
    /// A `JsonParseError` with a low-priority message suitable for fallback diagnostics.
    pub fn unmatched(expected: &str, input: &str) -> Self {
        Self::new(&format!("Expected '{expected}' literal"), 0, input)
    }
}
