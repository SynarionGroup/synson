/// Represents an error encountered while parsing JSON.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonParseError {
    pub message: String,
    pub position: usize,
}

impl JsonParseError {
    /// Creates a new JsonParseError.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error.
    /// * `position` - Byte position in the input where the error occurred.
    ///
    /// # Returns
    ///
    /// A new `JsonParseError` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::json_parse_error::JsonParseError;
    ///
    /// let err = JsonParseError::new("Unexpected token", 12);
    /// assert_eq!(err.message, "Unexpected token");
    /// ```
    pub fn new(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position,
        }
    }
}
