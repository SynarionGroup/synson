[![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)](https://github.com/SynarionGroup/synson)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/SynarionGroup/synson/actions/workflows/ci.yml/badge.svg)](https://github.com/SynarionGroup/synson/actions)

# Synson

**Synson** is a minimal, dependency-free JSON parser written in pure Rust.  
Built for simplicity, auditability, and full control over your JSON handling.

> ⚠️ This project is in `v0.2.0-alpha`: incomplete and subject to breaking changes.

---

## ✨ Features (alpha)

- Parses `null`, booleans, numbers, strings, arrays, and objects (including nested structures)
- Support for scientific notation in numbers (e.g., `1e3`, `-2.5E-2`)
- **Flexible parsing modes**: Strict (default) and tolerant modes
- Strict validation for malformed JSON (e.g., missing colons, trailing commas)
- No external dependencies
- Fully testable and auditable

---

## 🚀 Example

### Strict Mode (default)

Strict mode ensures compliance with JSON syntax, rejecting errors like trailing commas:

```rust
use synson::{parse_json, JsonValue};
use synson::model::JsonParseOptions;

let json = "{\"key\": {\"nested\": true}}";
let result = parse_json(json, Some(&JsonParseOptions::strict()));
assert!(result.is_ok());
```
---

### Tolerant Mode

Tolerant mode allows trailing characters and more lenient parsing:

```rust
let json = "{\"key\": [true, false,]}";
let result = parse_json(json, Some(&JsonParseOptions::tolerant()));
assert!(result.is_ok());
```

---

## 🔧 Roadmap

- [x] Basic types (null, bool, number, string)
- [x] Flat arrays and objects
- [x] Nested structures
- [ ] Unicode string support
- [ ] Serialization (to_json)

---

## 📦 License
MIT © Synarion Technologies – See [LICENSE](LICENSE) for more details.

---

## Credits

Developed by **Pierrick FONQUERNE**.
