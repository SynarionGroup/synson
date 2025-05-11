# Changelog

## [v0.2.0] - 2025-05-11

### ✨ Added

- **JsonParseOptions** is now optional in the `parse_json` function. If not provided, it defaults to strict mode.
- Parser now supports **strict** and **tolerant** modes:
  - **Strict mode**: Rejects trailing commas, malformed numbers, and extra characters after the JSON value.
  - **Tolerant mode**: Allows trailing characters, making it more lenient with non-compliant JSON.
- Improved error handling for malformed JSON:
  - Trailing characters after valid JSON values are now detected and reported.
  - Missing colons in object key-value pairs, trailing commas, unterminated strings, invalid escape sequences, and invalid number exponents are caught with detailed error messages.
- Enhanced error logging with position information (line, column, index) to facilitate debugging.

### ✅ JSON support

- Added support for deeply nested JSON structures via recursive `parse_value`.
- Arrays and objects can now contain any valid JSON value, including nested arrays and objects.
- **Scientific notation** in numbers is now supported (e.g., `1e3`, `-2.5E-2`).
- Full support for JSON string escape sequences (e.g., `\\`, `\"`, `\b`, `\n`, `\r`, `\t`).
- **Strict validation**: Rejects malformed JSON, including leading zeroes, incomplete exponents, and trailing commas in arrays or objects.
- **Tolerant mode**: Allows trailing characters after JSON values, increasing flexibility.

### 🧪 Test coverage

- Expanded unit and integration tests to cover:
  - Valid and invalid exponential numbers, edge cases, and malformed JSON.
  - Trailing characters after valid JSON values (e.g., `"true false"` or `"{...} extra"`).
  - Tests now reflect the ability to handle both **strict** and **tolerant** modes.
- Comprehensive error handling tests to verify that:
  - Missing colons, trailing commas, and unterminated strings trigger the correct errors.
  - Leading zeros and invalid escape sequences are handled as expected.
  
## [v0.1.0] - 2025-05-08

### ✨ Added

- `parse_json`: entry point for parsing complete JSON values.
- Primitive parsers: `parse_null`, `parse_bool`, `parse_number`, `parse_string`.
- `parse_array`: recursive parsing of arrays (e.g. `[1, "ok", false]`) with strict comma/bracket rules.
- `parse_object`: parsing of flat objects (e.g. `{ "a": 1, "b": "ok" }`) with strict syntax validation.
- Core types: `JsonValue` and `JsonParseError`.
- Centralized `parse_value` used across all composite structures.
- Full unit and integration test coverage.

### ✅ JSON support

- Supported types: `null`, `bool`, `number`, `string`, `array`, `object`.
- Escape handling: `\\`, `\"`, `\\n`, `\\t` in strings.
- Strict compliance with RFC 8259 (except for `\\uXXXX`, not yet implemented).

### 🧪 Test coverage

- Comprehensive unit tests for all types and edge cases.
- Integration tests for full JSON examples including nested arrays and objects.
- Error handling tests for syntax violations (missing `:`, trailing commas, unquoted keys, etc.)
