# Changelog

## [v0.2.0] - 2025-05-09

### ✨ Added

- Extended test suite for recursive parsing of nested arrays and objects.
- New integration tests validating combinations of arrays and objects at arbitrary depth.
- Improved documentation for `parse_object` to reflect support for nested structures.

### ✅ JSON support

- Confirmed support for deeply nested JSON structures via recursive `parse_value`.
- Arrays and objects can now contain any valid JSON value, including nested arrays/objects.

### 🧪 Test coverage

- Added tests for cases like `[{"a": [true, false]}, 3]` and `{"user": {"id": 1, "tags": ["rust", "json"]}}`.
- Added regression tests for deeply nested object trees (`{"a": {"b": {"c": {"d": null}}}}`).
- Clarified function behavior through test-driven validation of recursive parsing.

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
