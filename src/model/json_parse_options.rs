// src/model/json_parse_options.rs

/// Configuration options for the JSON parser.
/// This structure holds parsing configurations that control strictness and tolerance modes.
///
/// These options can be extended in future versions to support more features,
/// such as handling trailing commas, comments, and other parsing behaviors.
#[derive(Debug, Clone, Copy)]
pub struct JsonParseOptions {
    /// If true, the parser enforces strict parsing rules.
    /// For example, trailing commas and comments will be rejected in strict mode.
    pub strict: bool,

    /// If true, the parser allows trailing commas.
    /// This is useful for more lenient parsing, especially when dealing with JSON
    /// that may contain such syntax.
    pub allow_trailing_commas: bool,
}

impl JsonParseOptions {
    /// Creates a new instance of `JsonParseOptions` with the specified settings.
    ///
    /// # Arguments
    ///
    /// * `strict` - If the parser should be in strict mode. If true, it will reject any invalid formatting.
    /// * `allow_trailing_commas` - If the parser should allow trailing commas in arrays and objects.
    ///
    /// # Returns
    ///
    /// A new `JsonParseOptions` instance with the specified settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::JsonParseOptions;
    ///
    /// // Create a strict parser with no trailing comma allowance
    /// let options = JsonParseOptions::new(true, false);
    ///
    /// // Create a tolerant parser with trailing commas allowed
    /// let options = JsonParseOptions::new(false, true);
    /// ```
    pub fn new(strict: bool, allow_trailing_commas: bool) -> Self {
        JsonParseOptions {
            strict,
            allow_trailing_commas,
        }
    }

    /// Creates a new instance of `JsonParseOptions` with strict mode enabled.
    ///
    /// # Returns
    ///
    /// A `JsonParseOptions` instance with strict mode enabled and trailing commas disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::JsonParseOptions;
    ///
    /// let options = JsonParseOptions::strict();
    /// ```
    pub fn strict() -> Self {
        JsonParseOptions {
            strict: true,
            allow_trailing_commas: false,
        }
    }

    /// Creates a new instance of `JsonParseOptions` with tolerant mode enabled.
    ///
    /// # Returns
    ///
    /// A `JsonParseOptions` instance with tolerant mode enabled and trailing commas allowed.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::JsonParseOptions;
    ///
    /// let options = JsonParseOptions::tolerant();
    /// ```
    pub fn tolerant() -> Self {
        JsonParseOptions {
            strict: false,
            allow_trailing_commas: true,
        }
    }
}

impl Default for JsonParseOptions {
    /// Returns a `JsonParseOptions` instance with the default settings:
    /// - `strict` mode is `true` (strict validation).
    /// - `allow_trailing_commas` is `false` (trailing commas are not allowed).
    ///
    /// # Returns
    ///
    /// A new `JsonParseOptions` instance with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use synson::model::JsonParseOptions;
    ///
    /// let options = JsonParseOptions::default();
    /// ```
    fn default() -> Self {
        JsonParseOptions {
            strict: true,
            allow_trailing_commas: false,
        }
    }
}
