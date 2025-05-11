# Changelog

## [v0.2.0] - 2025-05-11

### ✨ Added

- Extended test suite for recursive parsing of nested arrays and objects.
- New integration tests validating combinations of arrays and objects at arbitrary depth.
- Improved documentation for `parse_object` to reflect support for nested structures.
- Support for scientific notation in numbers (e.g. `1e3`, `-2.5E-2`, `0.5e+2`).
- Stricter validation of JSON number syntax, rejecting invalid formats such as:
  - Leading zeroes (`01`, `00`)
  - Incomplete decimals (`5.`, `.5`)
  - Incomplete exponents (`1e`, `1e+`, `1e-`)
  - Unexpected trailing characters (`3.14rest`, `12abc`)
- Full support for JSON string escape sequences as per RFC 8259:
  - `\\`, `\"`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`
- Extended `parse_string` to decode all supported sequences and reject invalid ones.
- Improved error handling for JSON parsing:
  - Detailed error messages for parsing issues, including:
    - Trailing characters** after a valid JSON value.
    - Missing colons in object key-value pairs.
    - Trailing commas in arrays before the closing bracket.
    - Unterminated string literals and invalid escape sequences in strings.
    - Invalid number exponents and leading zeros in numbers.
- Enhanced error logging to include position information (line, column, index) for better debugging.
- Custom error handling for malformed JSON objects, arrays, and strings.
- Trailing characters rejection: The JSON parser now rejects any remaining characters after successfully parsing a JSON value. If there are extra characters, the parser will return a detailed error message indicating the invalid content.
  - Example errors:
    - `"truex"`: Invalid because of trailing `x` after the valid `true`.
    - `"{...} extra"`: Invalid because of the `extra` characters after a valid object.

### ✅ JSON support

- Confirmed support for deeply nested JSON structures via recursive `parse_value`.
- Arrays and objects can now contain any valid JSON value, including nested arrays/objects.
- Added support for scientific notation in numbers (e.g. `1e3`, `-2.5E-2`) per RFC 8259.
- String values now correctly handle all standard JSON escape sequences.
- Unicode escape support (`\uXXXX`) is planned for a future version.
- Added strict validation for JSON syntax, rejecting invalid formats:
  - Incomplete exponents (`1e`, `1e+`, `1e-`).
  - Leading zeroes (`01`, `00`).
  - Incomplete decimals (`5.`, `.5`).
  - Invalid escape sequences in strings.
  - Unexpected trailing characters after valid JSON values.
- Error reporting now includes specific messages for:
  - Missing colons in object entries.
  - Trailing commas in arrays.
  - Unterminated strings.
  - Invalid characters in booleans (`trueX`, `falseY`).
- Now ensures that after parsing a valid JSON value (null, boolean, number, string, array, or object), no extra characters are left in the input string.
- The parser will raise an error if there are characters that aren't part of the JSON value, improving strict compliance with JSON formatting.

### 🧪 Test coverage

- Added tests for cases like `[{"a": [true, false]}, 3]` and `{"user": {"id": 1, "tags": ["rust", "json"]}}`.
- Added regression tests for deeply nested object trees (`{"a": {"b": {"c": {"d": null}}}}`).
- Clarified function behavior through test-driven validation of recursive parsing.
- Added tests for valid and invalid exponential numbers.
- Added edge cases for boundary values (`-0`, `0.0`, etc.).
- Added unit tests for all supported string escape sequences (`\b`, `\f`, `\r`, `\/`, etc.).
- Test coverage improvements:
  - Regression tests for common error cases like trailing characters, missing colons, and invalid escape sequences.
  - Tests for leading zeroes and exponent errors in numbers.
  - Comprehensive test cases for malformed JSON input and edge cases, including:
    - Trailing characters after valid JSON values (`true false`).
    - Missing colons in objects (`{"key" "value"}`).
    - Invalid escape sequences in strings.
- Fixed test cases for malformed JSON to ensure accurate error handling.
- Added tests to validate that any extra characters in the input, after a valid JSON value, trigger an error.
- Regression tests for various cases like `"truex"` and `"{...} extra"` ensuring proper error reporting for trailing characters.

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
