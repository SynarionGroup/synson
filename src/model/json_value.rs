// src/model/json_value.rs

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

impl JsonValue {
    /// Serializes the JsonValue into a JSON-formatted String.
    ///
    /// # Returns
    /// A `String` representing the JSON-encoded form of the JsonValue.
    ///
    /// # Examples
    /// ```
    /// use synson::JsonValue;
    ///
    /// let val = JsonValue::Bool(true);
    /// assert_eq!(val.to_json(), "true");
    /// ```
    pub fn to_json(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => {
                let escaped = s
                    .chars()
                    .map(|c| match c {
                        '"' => "\\\"".to_string(),
                        '\\' => "\\\\".to_string(),
                        '\n' => "\\n".to_string(),
                        '\r' => "\\r".to_string(),
                        '\t' => "\\t".to_string(),
                        '\u{08}' => "\\b".to_string(),
                        '\u{0C}' => "\\f".to_string(),
                        c if c.is_control() => format!("\\u{:04x}", c as u32),
                        c => c.to_string(),
                    })
                    .collect::<String>();
                format!("\"{}\"", escaped)
            }
            JsonValue::Array(arr) => {
                let elements = arr
                    .iter()
                    .map(|v| v.to_json())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{}]", elements)
            }
            JsonValue::Object(obj) => {
                let mut keys = obj.keys().collect::<Vec<_>>();
                keys.sort();
                let members = keys
                    .iter()
                    .map(|k| {
                        format!(
                            "{}:{}",
                            JsonValue::String((*k).clone()).to_json(),
                            obj.get(*k).unwrap().to_json()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{{{}}}", members)
            }
        }
    }
}
