[![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue)](https://github.com/SynarionGroup/synson)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/SynarionGroup/synson/actions/workflows/ci.yml/badge.svg)](https://github.com/SynarionGroup/synson/actions)

# Synson

**Synson** is a minimal, dependency-free JSON parser written in pure Rust.  
Built for simplicity, auditability, and full control over your JSON handling.

> ⚠️ This project is in `v0.1.0-alpha`: incomplete and subject to breaking changes.

---

## ✨ Features (alpha)

- Parses `null`, booleans, numbers, strings, arrays, and flat objects
- No external dependencies
- Fully testable and auditable

---

## 🚀 Example

```rust
use synson::{parse_json, JsonValue};

let json = r#"{"ok": true, "count": 3}"#;
let value = parse_json(json).unwrap();

assert_eq!(
    value,
    JsonValue::Object(vec![
        ("ok".to_string(), JsonValue::Bool(true)),
        ("count".to_string(), JsonValue::Number(3.0)),
    ])
);
```

---

## 🔧 Roadmap

- [x] Basic types (null, bool, number, string)
- [x] Flat arrays and objects
- [ ] Nested structures
- [ ] Unicode string support
- [ ] Serialization (to_json)

---

## 📦 License
MIT © Synarion Technologies – See [LICENSE](LICENSE) for more details.

---

## Credits

Developed by **Pierrick FONQUERNE**.
