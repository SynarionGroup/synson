/// Represents a JSON value.
///
/// Variants cover all standard JSON types.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}

/// Represents an error encountered while parsing JSON.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonParseError {
    pub message: String,
    pub position: usize,
}
