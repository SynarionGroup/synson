# Changelog

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
