# Changelog

## [v0.2.0] - 2025-05-11

### ✨ Added

- **Recursive parsing of nested structures**: Deeply nested objects and arrays are now fully supported.
- **Scientific notation support**: Numbers like `1e3`, `-2.5E-2`, `0.5e+2` are correctly parsed.
- **Improved error handling**:
  - More detailed error messages with position information (line, column, index).
  - Support for trailing characters after valid JSON values is rejected with informative error messages.
- **Flexible parsing modes**:
  - Strict mode: Rejects trailing commas, malformed numbers, and unexpected characters.
  - Tolerant mode: Allows trailing characters, making it more lenient for non-compliant JSON.
- **JsonParseOptions is now optional**: If not provided, the parser defaults to strict mode.

### ✅ JSON support

- Confirmed support for deeply nested JSON structures via recursive `parse_value`.
- Arrays and objects can now contain any valid JSON value, including nested arrays/objects.
- Enhanced support for scientific notation in numbers per RFC 8259.
- Full support for standard JSON escape sequences in strings.

### 🧪 Test coverage

- Added extensive tests for deeply nested JSON structures (objects and arrays).
- Valid and invalid test cases for scientific notation in numbers.
- Regression tests for trailing characters after valid JSON values (e.g., `true false`, `{"key": "value"} extra`).
- Tests added for both strict and tolerant parsing modes.
- Enhanced unit and integration tests covering all edge cases of malformed JSON input.

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
