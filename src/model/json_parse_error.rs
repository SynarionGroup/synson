use std::error::Error;
use std::fmt;

/// Enumerates the kinds of parsing errors that may occur when decoding JSON.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    // ────────────────────────────────
    // Syntax-level expectations
    // ────────────────────────────────
    /// A colon `:` was expected (e.g., after a key in an object).
    ExpectedColon,

    /// A comma `,` was expected (e.g., between array elements or object members).
    ExpectedComma,

    /// A string key was expected but not found (e.g., in an object).
    ExpectedStringKey,

    /// A JSON value was expected (e.g., number, object, array, etc.).
    ExpectedValue,

    // ────────────────────────────────
    // Unexpected tokens or characters
    // ────────────────────────────────
    /// A closing bracket `]` was expected (e.g., to end an array).
    ExpectedBracket,

    /// A closing brace `}` was expected (e.g., to end an object).
    ExpectedBrace,

    /// An unexpected character was found (e.g., a `}` instead of `,`).
    UnexpectedChar(char),

    /// The input ended unexpectedly before completing a valid value.
    UnexpectedEof,

    // ────────────────────────────────
    // String-related issues
    // ────────────────────────────────
    /// An invalid escape sequence was encountered (e.g., `\q`).
    InvalidEscape,

    /// A malformed Unicode escape was encountered (e.g., `\uZZZZ`).
    InvalidUnicode,

    /// The string literal was not properly closed with a `"` character.
    UnterminatedString,

    // ────────────────────────────────
    // Number-related issues
    // ────────────────────────────────
    /// Decimal point is not followed by digits (e.g., `5.`).
    DecimalWithoutDigits,

    /// Exponent part is malformed (e.g., `1e`, `2e+`).
    InvalidExponent,

    /// Number format is invalid (e.g., trailing comma: `1,`).
    InvalidNumber,

    /// Integer has leading zeros, which are not allowed (e.g., `01`).
    LeadingZero,

    // ────────────────────────────────
    // Object-related issues
    // ────────────────────────────────
    /// An invalid object key was encountered (e.g., a number instead of a string).
    InvalidObjectKey,

    /// An object contains a duplicate key (e.g., `"a":1, "a":2`).
    DuplicateKey(String),

    /// A trailing comma was found before a closing brace or bracket (e.g., `[1,]`).
    TrailingComma,

    // ────────────────────────────────
    // Catch-all
    // ────────────────────────────────
    /// A parser-defined error with a custom message.
    Custom(String),
}

/// Represents a detailed error that occurred while parsing a JSON input.
#[derive(Debug, PartialEq)]
pub struct JsonParseError {
    /// Byte offset in the input where the error occurred.
    pub index: usize,

    /// Line number (1-based) where the error occurred.
    pub line: usize,

    /// Column number (1-based) on the line where the error occurred.
    pub column: usize,

    /// The kind of error that occurred.
    pub kind: ErrorKind,

    /// A short snippet of the input near the error for context.
    pub snippet: String,
}

impl JsonParseError {
    /// Creates a new `JsonParseError` with structured metadata.
    ///
    /// This function computes the line, column, and input snippet context
    /// from a raw byte offset in the original JSON input.
    ///
    /// # Arguments
    ///
    /// * `input` - The full input string that was being parsed.
    /// * `position` - The byte index in the input where the error occurred.
    /// * `kind` - The kind of error encountered (`ErrorKind`).
    ///
    /// # Returns
    ///
    /// A fully constructed `JsonParseError` including line, column, and snippet.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::{JsonParseError, ErrorKind};
    ///
    /// let input = "{ \"a\": tru }";
    /// let err = JsonParseError::new(input, 8, ErrorKind::UnexpectedChar('t'));
    ///
    /// assert_eq!(err.line, 1);
    /// assert_eq!(err.column, 9);
    /// assert!(err.snippet.contains("tru"));
    /// ```
    pub fn new(input: &str, position: usize, kind: ErrorKind) -> Self {
        let (line, column) = compute_line_column(input, position);
        let snippet = extract_snippet(input, position);

        Self {
            kind,
            index: position,
            line,
            column,
            snippet,
        }
    }
    /// Creates a generic fallback error for a failed literal match.
    ///
    /// This is typically used when attempting to match a fixed keyword like `"null"` or `"true"`,
    /// and no confident match was found.
    ///
    /// # Arguments
    ///
    /// * `expected` - The name of the literal that was expected (e.g., `"null"`).
    /// * `input` - The current slice of JSON input being parsed.
    ///
    /// # Returns
    ///
    /// A `JsonParseError` with kind `Custom`, starting at index 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::{JsonParseError, ErrorKind};
    ///
    /// let err = JsonParseError::unmatched("null", "not_null");
    ///
    /// assert_eq!(err.index, 0);
    /// assert!(matches!(err.kind, ErrorKind::Custom(ref msg) if msg.contains("Expected 'null'")));
    /// ```
    pub fn unmatched(expected: &str, input: &str) -> Self {
        Self::new(
            input,
            0,
            ErrorKind::Custom(format!("Expected '{expected}' literal")),
        )
    }
}

/// Computes the 1-based (line, column) from a byte offset in the input string.
///
/// This is used to generate human-readable error locations.
fn compute_line_column(input: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in input.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// Extracts a short context snippet around the given offset in the input.
///
/// This helps visualize where in the input the error occurred, useful for CLI feedback.
///
/// # Returns
///
/// A formatted snippet string like `"near '...error_here...'"`.
fn extract_snippet(input: &str, offset: usize) -> String {
    const CONTEXT: usize = 10;
    let start = offset.saturating_sub(CONTEXT);
    let end = usize::min(input.len(), offset + CONTEXT);
    let snippet = &input[start..end];
    format!("near '{}'", snippet.escape_default())
}

impl fmt::Display for JsonParseError {
    /// Formats the error as a user-friendly diagnostic message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {:?} ({})",
            self.line, self.column, self.kind, self.snippet
        )
    }
}

impl Error for JsonParseError {}
